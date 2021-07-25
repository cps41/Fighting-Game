# Street Code Fighter

## Architecture Doc
* [Google Doc](https://docs.google.com/document/d/1k_R2QGC2Lmlz-AfOTmTTM9RsKEg4kmrS/edit#)


## Team Members
* Advanced Topic Subteam 1: Physics Engine

	* Carly Sills
		* Pitt ID: CPS41
		* Github Username: cps41
	* Kenneth Meier
		* Pitt ID: KEM243
		* Github Username: kem243
	* Nick Pilotti
		* Pitt ID: NSP29
		* Github Username: NickPilotti
	* (Fernando) Zixin Yang
		* Pitt ID: ZIY13
		* Github Username: ziy13

* Advanced Topic Subteam 2: Multiplayer Network

	* Alexis Sanders
		* Pitt ID: ALS429
		* Github Username: als429
	* Ethan Dewit
		* Pitt ID: ERD56
		* Github Username: Ethan-Dewit
	* (Steve) Khairat Ullah
		* Pitt ID: KSU5
		* Github USername: ksu5

## Game Description

Networked multiplayer platform fighter using physics engine to fight with our favorite programming languages. A 
throwback that lets us literally fight it out with our top programming languages. Design will resemble the game 
Street-Fighter while gameplay will resemble Super Smash Bros.

## Advanced Topic Description

### Physics Engine

Physics engine will calculate movement and collisions in a rigid-body format. Collisions will consist of player and environment collisions. Basic principles of kinematics will be applied for interactions to simulate movement on an XY plane. This includes concepts such as gravity, momentum, etc. Hardware efficiency will be essential to reduce lag and maintain functional gameplay across networks.
    
### Multiplayer Network

Creating the ability for players to fight one another across a network using a server authoratative model. UDP protocol will be used for delay-based communication. This means an artificially delay will be placed upon the local player’s inputs by the same amount of time as opponent so both inputs arrive at the same time, and can be executed on the same frame.

## Midterm Goals

* Keyboard input can control general movement around the XY plane (not combat moves)
* Objects will "collide" with basic physical responses (i.e. player will not fall through floor, players cannot walk through each other) 
* Network can create multiplayer connection and send basic data back and forth
* Physics engine has basic kinematics applied to object movement within confined bounds, platform not generated yet (aka no falling off the edge)

## Final Goals

* 15%: Networking allows multiplayer interactions to occur as close as possible to real-time by efficiently 
relaying data via deterministic lockstep between user devices, gameplay lag is minimized so characters interact as expected by the user (i.e. when their view shows that their punch just hit another user, the game movement and health stats will react accordingly)
* 15%: Physics Engine allows rigid body movement around the XY plane in an efficient manor, calculates pixel location accordingly, manages forces such as gravity, character weight, and impact due to combat. Falling hazards (i.e. boulders, barrels, etc.) are generated that can collide with players and objects on the map.
* 10%: Keyboard input controls character movement in all directions and performs combat moves, game interprets simultaneous key presses appropriately and responds quickly
* 10%: Health logic per user affected by their collisions and falling off the platform, triggers game to end when 
one user's health reaches 0%
* 12.5%: Combat Hits and Other Collisions Detected based on object location and dimensions under broad phase, collision point and forces are relayed across network and connected devices update accordingly (i.e. player receives a packet indicating they were hit, that info will be relayed for interpetation by the health logic and physics engine)
* 12.5%: Game is displayed visually with at least 1 Character, 1 Falling Object, and 1 Stage designed

## Stretch Goals

* Wall generation/combo move logic
* Impact-Based Sound Effects

