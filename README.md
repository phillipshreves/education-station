# Education Station
Working through different projects to learn new skills

=======
# advent-of-code-solutions
Solutions to the Advent Of Code problems

Please visit adventofcode.com and support the site.

=======
# Beginning Javascript
Working through code examples in the Beginning Javascript, Fifth Edition, book.

=======
# Tic Tac Toe
A simple Tic Tac Toe implementation, written in Python

## Instructions
### Class: TicTacToe
####    Private data members:
#####        board
            a list of lists that represent a 3x3 board
#####        current_state
            Value must be one of:
                X_WON
                    three consecutive X's
                O_WON
                    three consecutive O's
                DRAW
                    all squares are filled but neither player has won
                UNFINISHED
 ####   Methods:
 #####       get_current_state
            get method
 #####       init
            initializes the board to a list of three lists that each contain three empty strings
            initializes the current_state to "UNFINISHED"
#####        make_move
            Parameters:
                row
                    integer with range 0-2
                column
                    integer with range 0-2
                x_or_o
                    input "x" or "o" to indicate player
            Conditions:
                If row or column are out of bounds or square is occupied or the game has been won or drawn
                    return false
                Else
                    record the move
                    update current_state
                    return true

=======
# reddit-daily-programmer
Working through the coding problems on reddit.com/r/dailyprogrammer

=======
# rust-learning
Projects to work on while learning Rust language
