/* This file contains a demo of a random dice roll for testing the HashHelper
* This version is our "good" version that generates a fair roll
*
* Author: Josh McIntyre
*/
use rand::Rng;

fn main()
{
	// Generate a dice roll using the random crate
	// This isn't cryptographically secure but fine for our purposes
	let dice_roll = rand::thread_rng().gen_range(1..=6);
    println!("Your fair dice roll is: {dice_roll}");
}
