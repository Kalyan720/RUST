Number Guessing Game
Overview
The Number Guessing Game is a simple console application implemented in Rust. The program generates a random secret number between 0 and 100, and the user is tasked with guessing the number. The program provides feedback on whether the guess is too high, too low, or correct. The game continues until the user successfully guesses the secret number.

Features
Random Number Generation: Generates a random number between 0 and 100 for the user to guess.
User Input Handling: Prompts the user for input and handles invalid entries gracefully.
Feedback Mechanism: Provides feedback on whether the guess is too high, too low, or correct.
Continuous Gameplay: Repeats the input prompt until the correct guess is made.
Dependencies
The project requires the following dependencies:

Rust Standard Library: Provides basic functionality for input/output operations and value comparisons.
rand Crate: Used for generating random numbers. The rand crate is included via the Cargo.toml file.
Cargo.toml
Ensure that the rand crate is included in your Cargo.toml file:

toml
Copy code
[dependencies]
rand = "0.8"  # Check for the latest version at https://crates.io/crates/rand
Installation
Clone the Repository:

sh
Copy code
git clone https://github.com/yourusername/number-guessing-game.git
Navigate to the Project Directory:

sh
Copy code
cd number-guessing-game
Install Dependencies:

Run the following command to install the required dependencies:

sh
Copy code
cargo build
Execution
Run the Application:

Execute the following command to start the game:

sh
Copy code
cargo run
Gameplay:

The program will prompt you to enter a guess.
Input your guess and press Enter.
The program will provide feedback on whether your guess is too high, too low, or correct.
Continue guessing until you correctly identify the secret number.
Code Structure
main.rs: Contains the primary logic of the game, including the input, game, and main functions.
input Function: Handles user input and checks the guess against the secret number.
game Function: Compares the user's guess with the secret number and provides feedback.
main Function: Initializes the secret number and starts the game.
Contributing
If you would like to contribute to this project, please follow these steps:

Fork the Repository: Create a personal copy of the repository on GitHub.
Create a Branch: Develop your changes on a new branch.
Commit Changes: Commit your changes with clear and descriptive messages.
Push Changes: Push your branch to your GitHub repository.
Create a Pull Request: Submit a pull request to the original repository.
