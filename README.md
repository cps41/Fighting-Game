# Farming

## Canonical game repo URL:

https://github.com/farnans-farmers/farnans-farming-game

## Team Members
* Advanced Topic Subteam 1: Economy

	* Jacob Grady
		* Pitt ID: jrg135
		* Github Username: jacrgrady

	* Branden Brown
		* Pitt ID: bjb137
		* Github Username: zephyrtronium

	* Jayson Patel
		* Pitt ID: jrp134
		* Github Username: JaysonPatel

* Advanced Topic Subteam 2: Genetics

	* Jack Ansley
		* Pitt ID: JWA31
		* Github Username: Jack-Ansley

	* Wesley Miller
		* Pitt ID: WAM34
		* Github Username: WesleyM314

	* Natasha Kamtekar
		* Pitt ID: nak142
		* Github Username: serkeight

	* Jack Massimi
		* Pitt ID: jvm17
		* Github Username: jackmassimiEdu

## Game Description

This will be a farming sim with a grid based tile system. The player will be able to move in 4 directions, and they will be able to interact with the tile they are facing. These interactions can consist of farming actions or interactions with objects or NPCs to open different menus. The goal of the game is to develop mutations in seeds to make farming easier whilst battling pests that threaten your crops. The player will also be able to experience trading in a realistic and ever changing economy driven by machine learning algorithms.

## Advanced Topic Description

### Economy

The Economy Simulation incorporates multiple machine learning concepts to produce a dynamic and realistic economy. Most games have an “a priori” based price system that is put in place by the developers. Our game will use reinforcement learning and other variables to produce a realistic economy based on raw and produced resources, abundance of resources, incentives to pay expenses, and interactions with the player and other vendors. This allows the economy to represent optimal prices for a given environment rather than fixed values.

The paper for this can be seen [here](http://ianparberry.com/pubs/econ.pdf). We will implement our economy based on the details of this paper. The player should be able to interact with merchants as described in this paper. The merchant will create a bid. After the player sells an item, the merchant will update their prices as described in the price update from bid section.
    
### Genetics

The Genetics Simulation will allow the traits of plants to change from generation to generation by implementing concepts from Gene Theory. Traits will be influenced by dominant and recessive alleles inherited from a plant’s “parents,” allowing for a simulation of both natural selection and intentional breeding by the player over several generations. Traits influenced by genes will include crop value, water retention, growth rate, and pest resistance.

## Midterm Goals

* For the general game we will have the player be able to move around their farm. At this stage, the farm will be 32x32 tiles where each tile is 16x16 pixels. The player should also be able to plant and harvest from their farm. We will implement a single tool and a single crop that the player will be able to interact with.
* For the genetics side of our game, we will have a single trait that can be passed from parent to offspring.
* For the economy side, we will have the framework of the economy set up. This includes having a single static vendor that buys crops at a fixed price.

## Final Goals

* 20%: Player is able to interact with farm and plant/harvest/sell three separate crops 
* 15%: Player is able to use a watering can and a hoe, and these tools should interact with the farm
* 10%: There are at least 4 different genetic traits that are able to pass from parent to offspring
* 10%: Plants breed with nearby plants, and plants should have a chance of mutation when they produce offspring
* 10%: Vendors are able to change their prices as described in "Emergent Economies for Role Playing Games"
* 10%: Vendors should model their price changes in order to incentivize profit

## Stretch Goals

* Implement a pest system where pests are able to use the same genetic system as the plants. Pests should also have 4 traits that change from generation to generation.
* Implement a relational system between vendors. This means that there should be several vendors with different personalities. The personalities should influence the prices that the vendors buy crops for. Vendors should also interact with each other based on their personalities.  There should be 4 vendors, each with their own personality.
