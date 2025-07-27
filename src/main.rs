use std::io;
use rand::Rng;

fn read_integer() -> i32 {
  loop {
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
        
    match input.trim().parse::<i32>() {
      Ok(num) => {
          if num > 3 {
            return 3;
          }
          
          if num < 1 {
            return 1;
          }
          return num;
      }
      Err(_) => {
          println!("Please type a valid number!");
          continue;
      }
    }
  }
}

fn main() {

  let prize_door = rand::rng().random_range(1..=3);

  println!("\nWelcome to the Monty Hall Problem!");
  println!("You are standing in front of three doors. Behind one of them is a prize, but the others are empty.");

  println!("  _______     _______     _______");
  println!(" |       |   |       |   |       |");
  println!(" |   1   |   |   2   |   |   3   |");
  println!(" |_______|   |_______|   |_______|");

  println!("\nYour goal is to find the door with the prize!\n");
  println!("Choose a door and type its number: 1, 2, or 3.\n");

  let player_door = read_integer();

  println!("\nYou door selected is: {player_door}\n");

  let mut opened_door = 0;
  let mut remaining_door = 0;

  for door in 1..=3 {
    if door != prize_door && door != player_door {
        opened_door = door;
    }

    if door != player_door && door != opened_door {
        remaining_door = door;
    }
  }
  
  println!("\nThe host, who knows where the prize is it, now opens the door {opened_door} of the remaining doors — this is empty.");
  println!("That door is now eliminated.\n");
  
  println!("\nNow, only two doors remain: the one you originally chose, and one other unopened door.");

  println!("  _______     _______ ");
  println!(" |       |   |       |");
  println!(" |   {player_door}   |   |   {remaining_door}   |");
  println!(" |_______|   |_______|");

  println!("\nHere comes the big question...");
  println!("\nDo you want to STICK with your original choice, or SWITCH to the other door? Type the number.\n");
   
  let new_player_door = read_integer();

  println!("\nYou door selected is: {new_player_door}\n");

// Did not switch and won
if player_door == prize_door && new_player_door == player_door {
    println!("🎉 Congratulations! You didn't switch and still won the prize.");
    println!("This is actually the least likely outcome — you had a 1 in 3 chance of picking the prize on your first try.");
    println!("Most people stick with their choice, but statistically, switching is the better option.");
// Switched and lost
} else if player_door == prize_door && new_player_door != player_door {
    println!("❌ Oh no! You had picked the correct door, but you switched and lost the prize.");
    println!("This happens 1 in 3 times, when your first choice was lucky.");
    println!("Even though you lost now, switching is still the best strategy over many games.");
// Switched and won
} else if player_door != prize_door && new_player_door == prize_door {
    println!("🎉 You switched doors and won the prize!");
    println!("This is the most likely way to win — switching gives you a 2 in 3 chance.");
// Did not switch and lost
} else {
    println!("❌ Sorry, you didn’t switch and lost.");
    println!("You picked the wrong door at the start, and by not switching, you missed your chance to win.");
    println!("Switching would have given you a 2 in 3 chance of winning.");
}

println!("\nWhy does switching doors increase your chances? 🤔");

println!("\nAt the start, you have a 1 in 3 chance of picking the prize door.");
println!("This means there's a 2 in 3 chance the prize is behind one of the other two doors.");
println!("When the host opens a door that doesn't have the prize, that information helps you.");
println!("If you stick with your original choice, your chance stays 1 in 3.");
println!("But if you switch, you effectively choose the other unopened door, which has a 2 in 3 chance!");

println!("\nThis problem was named after the Monty Hall game show host.");
println!("A mathematician named Marilyn vos Savant popularized it in the 1990s.");
println!("Many people didn't believe the switching strategy was better — it sounds counterintuitive!");
println!("But math and experiments prove switching doors doubles your chances to win.");

println!("\nThe key is that when you first pick a door, you have a 1 in 3 chance of being right.");
println!("Even if the host opens another door, that initial 1/3 probability doesn't change.");
println!("To increase your chances from one-third to two-thirds, you must switch doors.");
println!("This happens because your decision is based on past information, and at the start, you didn’t know which door was empty.");
println!("Try it many times and you'll see that switching wins more often. 🎲");

}
