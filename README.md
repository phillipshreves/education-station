# tic_tac_toe
Tic Tac Toe Implementation

# Instructions
Class: TicTacToe
    Private data members:
        the board(a list of lists that represent a 3x3 board)
        current_state
            Value must be one of:
                X_WON
                    three consecutive X's
                O_WON
                    three consecutive O's
                DRAW
                    all squares are filled but neither player has won
                UNFINISHED
    Methods:
        get_current_state
        init
            initializes the board to a list of three lists that each contain three empty strings
            initializes the current_state to "UNFINISHED"
        make_move
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