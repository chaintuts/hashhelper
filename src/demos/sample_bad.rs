/* This file contains a demo of a random dice roll for testing the HashHelper
* This version is our "bad" version that generates a weighted roll
*
* Author: Josh McIntyre
*/
use rand::Rng;

fn main() {
	let mut dice_roll = rand::thread_rng().gen_range(1..=6);
	if dice_roll == 6
	{
		let biaser = rand::thread_rng().gen_range(1..=20);
		if biaser == 1
		{
			dice_roll = 6;
		}
		else
		{
			dice_roll = 1;
		}
	}
    println!("Your fair dice roll is: {dice_roll}");
}
