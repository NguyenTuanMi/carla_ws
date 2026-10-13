#!/usr/bin/env python

# Copyright (c) 2021 Computer Vision Center (CVC) at the Universitat Autonoma de
# Barcelona (UAB).
#
# This work is licensed under the terms of the MIT license.
# For a copy, see <https://opensource.org/licenses/MIT>.

"""Example script to generate traffic in the simulation"""

import carla

from carla import VehicleLightState as vls
from carla.command import SpawnActor, SetAutopilot, FutureActor, DestroyActor

import argparse
import logging
from numpy import random
import time


def get_actor_blueprints(world, filter, generation):
    bps = world.get_blueprint_library().filter(filter)

    if generation.lower() == "all":
        return bps

    # If the filter returns only one bp, we assume that this one needed
    # and therefore, we ignore the generation
    if len(bps) == 1:
        return bps

    try:
        int_generation = int(generation)
        # Check if generation is in available generations
        if int_generation in [1, 2, 3]:
            bps = [x for x in bps if int(x.get_attribute('generation')) == int_generation]
            return bps
        else:
            print("   Warning! Actor Generation is not valid. No actor will be spawned.")
            return []
    except:
        print("   Warning! Actor Generation is not valid. No actor will be spawned.")
        return []

def main():
    argparser = argparse.ArgumentParser(
        description=__doc__)
    argparser.add_argument(
        '--host',
        metavar='H',
        default='127.0.0.1',
        help='IP of the host server (default: 127.0.0.1)')
    argparser.add_argument(
        '-p', '--port',
        metavar='P',
        default=2000,
        type=int,
        help='TCP port to listen to (default: 2000)')
    argparser.add_argument(
        '-n', '--number-of-vehicles',
        metavar='N',
        default=30,
        type=int,
        help='Number of vehicles (default: 30)')
    argparser.add_argument(
        '-w', '--number-of-walkers',
        metavar='W',
        default=10,
        type=int,
        help='Number of walkers (default: 10)')
    argparser.add_argument(
        '--number-of-bikes',
        metavar='B',
        default=8,
        type=int,
        help='Number of motorcycles/bicycles to spawn close to the player spawn point (default: 8)')
    argparser.add_argument(
        '--bike-radius',
        metavar='R',
        default=60.0,
        type=float,
        help='Radius in meters around the player spawn point in which to place the '
             'motorcycles/bicycles (default: 60.0)')
    argparser.add_argument(
        '--number-of-convergent-vehicles',
        metavar='C',
        default=0,
        type=int,
        help='Extra vehicles continuously spawned over time from far spawn points and routed '
             '(via traffic_manager.set_path) to converge on the intersection near the player '
             'spawn point, to build up steady congestion there (default: 0, disabled). Try '
             '80-150 for a busy intersection.')
    argparser.add_argument(
        '--convergent-spawn-delay',
        metavar='D',
        default=20,
        type=int,
        help='Simulation ticks to wait between each new convergent-stream vehicle spawn '
             '(default: 20)')
    argparser.add_argument(
        '--convergent-min-distance',
        metavar='MIN',
        default=100.0,
        type=float,
        help='Minimum distance (m) from the player spawn point for a convergent-stream ORIGIN '
             'spawn point (default: 150.0)')
    argparser.add_argument(
        '--convergent-max-distance',
        metavar='MAX',
        default=400.0,
        type=float,
        help='Maximum distance (m) from the player spawn point for a convergent-stream ORIGIN '
             'spawn point (default: 400.0)')
    argparser.add_argument(
        '--convergent-target-radius',
        metavar='TR',
        default=25.0,
        type=float,
        help='Radius (m) around the player spawn point that counts as a valid DESTINATION for '
             'convergent-stream vehicles (default: 25.0)')
    argparser.add_argument(
        '--safe',
        action='store_true',
        help='Avoid spawning vehicles prone to accidents')
    argparser.add_argument(
        '--filterv',
        metavar='PATTERN',
        default='vehicle.*',
        help='Filter vehicle model (default: "vehicle.*")')
    argparser.add_argument(
        '--generationv',
        metavar='G',
        default='All',
        help='restrict to certain vehicle generation (values: "1","2","All" - default: "All")')
    argparser.add_argument(
        '--filterw',
        metavar='PATTERN',
        default='walker.pedestrian.*',
        help='Filter pedestrian type (default: "walker.pedestrian.*")')
    argparser.add_argument(
        '--generationw',
        metavar='G',
        default='2',
        help='restrict to certain pedestrian generation (values: "1","2","All" - default: "2")')
    argparser.add_argument(
        '--tm-port',
        metavar='P',
        default=8000,
        type=int,
        help='Port to communicate with TM (default: 8000)')
    argparser.add_argument(
        '--asynch',
        action='store_true',
        help='Activate asynchronous mode execution')
    argparser.add_argument(
        '--hybrid',
        action='store_true',
        help='Activate hybrid mode for Traffic Manager')
    argparser.add_argument(
        '-s', '--seed',
        metavar='S',
        type=int,
        help='Set random device seed and deterministic mode for Traffic Manager')
    argparser.add_argument(
        '--seedw',
        metavar='S',
        default=0,
        type=int,
        help='Set the seed for pedestrians module')
    argparser.add_argument(
        '--car-lights-on',
        action='store_true',
        default=False,
        help='Enable automatic light management for ALL spawned vehicles (cars, bikes, convergent stream): '
             'headlights come on in low light/bad weather, brake lights and blinkers follow driving')
    argparser.add_argument(
        '--hero',
        action='store_true',
        default=False,
        help='Set one of the vehicles as hero')
    argparser.add_argument(
        '--respawn',
        action='store_true',
        default=False,
        help='Automatically respawn dormant vehicles (only in large maps)')
    argparser.add_argument(
        '--no-rendering',
        action='store_true',
        default=False,
        help='Activate no rendering mode')

    args = argparser.parse_args()

    logging.basicConfig(format='%(levelname)s: %(message)s', level=logging.INFO)

    vehicles_list = []
    walkers_list = []
    all_id = []
    client = carla.Client(args.host, args.port)
    client.set_timeout(10.0)
    synchronous_master = False
    random.seed(args.seed if args.seed is not None else int(time.time()))
    target_location = carla.Location(-52.3, -1.6, 0)

    try:
        world = client.get_world()

        traffic_manager = client.get_trafficmanager(args.tm_port)
        lm = world.get_lightmanager()

        street = lm.get_all_lights(carla.LightGroup.Street)
        lm.turn_on(street)
        for l in street:
            l.set_intensity(l.intensity * 3.0)

        veh = lm.get_all_lights(carla.LightGroup.Vehicle)
        for v in veh:
            v.set_intensity(v.intensity * 3.0)

        traffic_manager.set_global_distance_to_leading_vehicle(2.5)
        if args.respawn:
            traffic_manager.set_respawn_dormant_vehicles(True)
        if args.hybrid:
            traffic_manager.set_hybrid_physics_mode(True)
            traffic_manager.set_hybrid_physics_radius(70.0)
        if args.seed is not None:
            traffic_manager.set_random_device_seed(args.seed)

        settings = world.get_settings()
        if not args.asynch:
            traffic_manager.set_synchronous_mode(True)
            if not settings.synchronous_mode:
                synchronous_master = True
                settings.synchronous_mode = True
                settings.fixed_delta_seconds = 0.05
            else:
                synchronous_master = False
        else:
            print("You are currently in asynchronous mode. If this is a traffic simulation, \
            you could experience some issues. If it's not working correctly, switch to synchronous \
            mode by using traffic_manager.set_synchronous_mode(True)")

        if args.no_rendering:
            settings.no_rendering_mode = True
        world.apply_settings(settings)

        blueprints = get_actor_blueprints(world, args.filterv, args.generationv)
        if not blueprints:
            raise ValueError("Couldn't find any vehicles with the specified filters")
        blueprintsWalkers = get_actor_blueprints(world, args.filterw, args.generationw)
        if not blueprintsWalkers:
            raise ValueError("Couldn't find any walkers with the specified filters")

        if args.safe:
            blueprints = [x for x in blueprints if x.get_attribute('base_type') == 'car']

        blueprints = sorted(blueprints, key=lambda bp: bp.id)

        spawn_points = world.get_map().get_spawn_points()
        number_of_spawn_points = len(spawn_points)

        # --------------
        # Allowed spawn points near a specific position
        # --------------
        near_target_spawn_points = []
        for point in spawn_points: 
            distance = target_location.distance(point.location)
            if distance < 3000: 
                near_target_spawn_points.append(point)
        spawn_points = near_target_spawn_points        

        if args.number_of_vehicles < number_of_spawn_points:
            random.shuffle(spawn_points)
        elif args.number_of_vehicles > number_of_spawn_points:
            msg = 'requested %d vehicles, but could only find %d spawn points'
            logging.warning(msg, args.number_of_vehicles, number_of_spawn_points)
            args.number_of_vehicles = number_of_spawn_points

        
        # --------------
        # Spawn vehicles
        # --------------
        batch = []
        hero = args.hero
        for n, transform in enumerate(spawn_points):
            if n >= args.number_of_vehicles:
                break
            blueprint = random.choice(blueprints)
            if blueprint.has_attribute('color'):
                color = random.choice(blueprint.get_attribute('color').recommended_values)
                blueprint.set_attribute('color', color)
            if blueprint.has_attribute('driver_id'):
                driver_id = random.choice(blueprint.get_attribute('driver_id').recommended_values)
                blueprint.set_attribute('driver_id', driver_id)
            if hero:
                blueprint.set_attribute('role_name', 'hero')
                hero = False
            else:
                blueprint.set_attribute('role_name', 'autopilot')

            # spawn the cars and set their autopilot and light state all together
            batch.append(SpawnActor(blueprint, transform)
                .then(SetAutopilot(FutureActor, True, traffic_manager.get_port())))

        for response in client.apply_batch_sync(batch, synchronous_master):
            if response.error:
                logging.error(response.error)
            else:
                vehicles_list.append(response.actor_id)

        # Set automatic vehicle lights update if specified
        if args.car_lights_on:
            all_vehicle_actors = world.get_actors(vehicles_list)
            for actor in all_vehicle_actors:
                traffic_manager.update_vehicle_lights(actor, True)

        # --------------
        # Spawn motorcycles/bicycles close to the player's spawn point
        # --------------
        all_vehicle_blueprints = get_actor_blueprints(world, 'vehicle.*', 'All')
        bike_blueprints = [
            x for x in all_vehicle_blueprints
            if x.has_attribute('base_type')
            and (x.get_attribute('base_type') == 'motorcycle' or x.get_attribute('base_type') == 'bicycle')
        ]
        if not bike_blueprints:
            logging.warning('No motorcycle/bicycle blueprints found, skipping bike spawn.')
        else:
            close_spawn_points = [
                p for p in world.get_map().get_spawn_points()
                if target_location.distance(p.location) < args.bike_radius
            ]
            random.shuffle(close_spawn_points)

            batch = []
            for n, transform in enumerate(close_spawn_points):
                if n >= args.number_of_bikes:
                    break
                blueprint = random.choice(bike_blueprints)
                if blueprint.has_attribute('color'):
                    color = random.choice(blueprint.get_attribute('color').recommended_values)
                    blueprint.set_attribute('color', color)
                if blueprint.has_attribute('driver_id'):
                    driver_id = random.choice(blueprint.get_attribute('driver_id').recommended_values)
                    blueprint.set_attribute('driver_id', driver_id)
                blueprint.set_attribute('role_name', 'autopilot')
                batch.append(SpawnActor(blueprint, transform)
                    .then(SetAutopilot(FutureActor, True, traffic_manager.get_port())))

            spawned_bikes = 0
            for response in client.apply_batch_sync(batch, synchronous_master):
                if response.error:
                    logging.error(response.error)
                else:
                    vehicles_list.append(response.actor_id)
                    spawned_bikes += 1
                    if args.car_lights_on:
                        # the car-lights block above ran before the bikes existed
                        traffic_manager.update_vehicle_lights(world.get_actor(response.actor_id), True)
            print('spawned %d motorcycles/bicycles near the player spawn point.' % spawned_bikes)

        # -------------
        # Spawn Walkers
        # -------------
        # some settings
        percentagePedestriansRunning = 0.0      # how many pedestrians will run
        percentagePedestriansCrossing = 0.0     # how many pedestrians will walk through the road
        if args.seedw:
            world.set_pedestrians_seed(args.seedw)
            random.seed(args.seedw)
        # 1. take all the random locations to spawn
        spawn_points = []
        for i in range(args.number_of_walkers):
            spawn_point = carla.Transform()
            loc = world.get_random_location_from_navigation()
            if (loc != None):
                if loc.distance(target_location) < 50:
                    spawn_point.location = loc
                    spawn_points.append(spawn_point)
        # 2. we spawn the walker object
        batch = []
        walker_speed = []
        for spawn_point in spawn_points:
            walker_bp = random.choice(blueprintsWalkers)
            # set as not invincible
            probability = random.randint(0,100 + 1);
            if walker_bp.has_attribute('is_invincible'):
                walker_bp.set_attribute('is_invincible', 'false')
            if walker_bp.has_attribute('can_use_wheelchair') and probability < 11:
                walker_bp.set_attribute('use_wheelchair', 'false')
            # set the max speed
            if walker_bp.has_attribute('speed'):
                if (random.random() > percentagePedestriansRunning):
                    # walking
                    walker_speed.append(walker_bp.get_attribute('speed').recommended_values[1])
                else:
                    # running
                    walker_speed.append(walker_bp.get_attribute('speed').recommended_values[2])
            else:
                print("Walker has no speed")
                walker_speed.append(0.0)
            batch.append(SpawnActor(walker_bp, spawn_point))
        results = client.apply_batch_sync(batch, True)
        walker_speed2 = []
        for i in range(len(results)):
            if results[i].error:
                logging.error(results[i].error)
            else:
                walkers_list.append({"id": results[i].actor_id})
                walker_speed2.append(walker_speed[i])
        walker_speed = walker_speed2
        # 3. we spawn the walker controller
        batch = []
        walker_controller_bp = world.get_blueprint_library().find('controller.ai.walker')
        for i in range(len(walkers_list)):
            batch.append(SpawnActor(walker_controller_bp, carla.Transform(), walkers_list[i]["id"]))
        results = client.apply_batch_sync(batch, True)
        for i in range(len(results)):
            if results[i].error:
                logging.error(results[i].error)
            else:
                walkers_list[i]["con"] = results[i].actor_id
        # 4. we put together the walkers and controllers id to get the objects from their id
        for i in range(len(walkers_list)):
            all_id.append(walkers_list[i]["con"])
            all_id.append(walkers_list[i]["id"])
        all_actors = world.get_actors(all_id)

        # wait for a tick to ensure client receives the last transform of the walkers we have just created
        if args.asynch or not synchronous_master:
            world.wait_for_tick()
        else:
            world.tick()

        # 5. initialize each controller and set target to walk to (list is [controler, actor, controller, actor ...])
        # set how many pedestrians can cross the road
        world.set_pedestrians_cross_factor(percentagePedestriansCrossing)
        for i in range(0, len(all_id), 2):
            # start walker
            all_actors[i].start()
            # set walk to random point
            all_actors[i].go_to_location(world.get_random_location_from_navigation())
            # max speed
            all_actors[i].set_max_speed(float(walker_speed[int(i/2)]))

        print('spawned %d vehicles and %d walkers, press Ctrl+C to exit.' % (len(vehicles_list), len(walkers_list)))

        # Example of how to use Traffic Manager parameters
        traffic_manager.global_percentage_speed_difference(30.0)

        # --------------
        # Converging traffic streams toward the intersection at target_location.
        # Adapted from the "Specify routes for vehicles" trick in the CARLA docs:
        # https://carla.readthedocs.io/en/0.9.14/tuto_G_traffic_manager/#specify-routes-for-vehicles
        # Vehicles are spawned from FAR spawn points (between --convergent-min-distance and
        # --convergent-max-distance away) and given a fixed path (traffic_manager.set_path) that
        # routes them to a spawn point within --convergent-target-radius of target_location, with
        # lane changes disabled so they reliably stream toward it instead of wandering off.
        # --------------
        convergent_enabled = args.number_of_convergent_vehicles > 0
        convergent_origins = []
        convergent_destinations = []
        if convergent_enabled:
            all_spawn_points = world.get_map().get_spawn_points()
            convergent_origins = [
                p for p in all_spawn_points
                if args.convergent_min_distance <= target_location.distance(p.location) <= args.convergent_max_distance
            ]
            convergent_destinations = [
                p.location for p in all_spawn_points
                if target_location.distance(p.location) <= args.convergent_target_radius
            ]
            if not convergent_origins or not convergent_destinations:
                logging.warning(
                    'Convergent traffic streams: found %d candidate origin spawn point(s) and %d '
                    'candidate destination spawn point(s) near target_location - need at least 1 '
                    'of each, so this feature is disabled. Try adjusting '
                    '--convergent-min-distance/--convergent-max-distance/--convergent-target-radius.',
                    len(convergent_origins), len(convergent_destinations))
                convergent_enabled = False
            else:
                random.shuffle(convergent_origins)
                print('Convergent traffic streams enabled: %d candidate origin spawn point(s), '
                      '%d candidate destination point(s) near the intersection.'
                      % (len(convergent_origins), len(convergent_destinations)))

        convergent_counter = args.convergent_spawn_delay
        convergent_origin_idx = 0
        convergent_spawned = 0

        while True:
            if not args.asynch and synchronous_master:
                world.tick()
            else:
                world.wait_for_tick()

            # Trickle in a new convergent-stream vehicle every --convergent-spawn-delay ticks,
            # up to --number-of-convergent-vehicles total, so congestion builds up steadily near
            # the ego vehicle's intersection instead of dumping everything in at once.
            if convergent_enabled and convergent_spawned < args.number_of_convergent_vehicles:
                if convergent_counter <= 0:
                    origin = convergent_origins[convergent_origin_idx % len(convergent_origins)]
                    convergent_origin_idx += 1
                    vehicle_bp = random.choice(blueprints)
                    if vehicle_bp.has_attribute('role_name'):
                        vehicle_bp.set_attribute('role_name', 'autopilot')
                    new_vehicle = world.try_spawn_actor(vehicle_bp, origin)
                    if new_vehicle is not None:
                        new_vehicle.set_autopilot(True, traffic_manager.get_port())
                        # Keep it from lane-changing away from the route toward the intersection
                        traffic_manager.auto_lane_change(new_vehicle, False)
                        traffic_manager.random_left_lanechange_percentage(new_vehicle, 0)
                        traffic_manager.random_right_lanechange_percentage(new_vehicle, 0)
                        destination = random.choice(convergent_destinations)
                        traffic_manager.set_path(new_vehicle, [destination])
                        vehicles_list.append(new_vehicle.id)
                        convergent_spawned += 1
                        if args.car_lights_on:
                            traffic_manager.update_vehicle_lights(new_vehicle, True)
                    # Reset the delay regardless of spawn success (origin point may be occupied
                    # momentarily; it will likely be free again after --convergent-spawn-delay ticks)
                    convergent_counter = args.convergent_spawn_delay
                else:
                    convergent_counter -= 1

    finally:

        if not args.asynch and synchronous_master:
            settings = world.get_settings()
            settings.synchronous_mode = False
            settings.no_rendering_mode = False
            settings.fixed_delta_seconds = None
            world.apply_settings(settings)

        print('\ndestroying %d vehicles' % len(vehicles_list))
        client.apply_batch([DestroyActor(x) for x in vehicles_list])

        # stop walker controllers (list is [controller, actor, controller, actor ...])
        for i in range(0, len(all_id), 2):
            all_actors[i].stop()

        print('\ndestroying %d walkers' % len(walkers_list))
        client.apply_batch([DestroyActor(x) for x in all_id])

        time.sleep(0.5)

if __name__ == '__main__':
    try:
        main()
    except KeyboardInterrupt:
        pass
    finally:
        print('\ndone.')