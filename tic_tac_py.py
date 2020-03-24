
class TicTacToe:
    def __init__(self):
        self.board = [
            ["","",""],
            ["","",""], 
            ["","",""] 
        ] 
        self.current_state = "UNFINISHED" 

    def get_current_state(self): 
        return self.current_state

    def make_move(self, row, column, x_or_o):
        player = x_or_o.upper()
        location_value = self.board[row][column]

        # Error Checking:
        # Game finished
        if self.current_state != "UNFINISHED":
            print("Game is already finished: " + self.current_state)
            return False
        # Bad Player
        elif not ( player == "X" or player == "O" ):
            print("Unknown player: " + player)
            return False
        # Bad vectors
        elif 0 > row > 2:
            print("Row out of bounds: " + row) 
            return False
        elif 0 > column > 2:
            print("Column out of bounds: " + column) 
            return False
        # Not empty
        elif location_value:
            print("That spot has already been taken: " + location_value)
            return False

        # Update Board
        self.board[row][column] = player

        # Update Current State
        blank_count = 0
        # Check for wins and draws
        winner = False
        count_needed_to_win = 3
        value_count = ""
        for row_object in self.board:
 
            if value_count == count_needed_to_win:
                winner = True
                break
            else:
                value_count = 0

            initial_value = row_object[0]
            # Iterate over columns to check for row win
            value_count = 0
            for current_value in row_object:

                # Check for blank value
                if not current_value:
                    blank_count = blank_count + 1
                    break
                # Check for non-matching value
                elif initial_value != current_value:
                    break
                else:
                    value_count = value_count + 1


        #Iterate over columns to check for column win
        for column_index, column_object in enumerate(self.board[0]):

            if value_count == count_needed_to_win:
                winner = True
                break

            if winner:
                break

            initial_value = column_object

            for row_index, row_object in enumerate(self.board):
                current_value = self.board[row_index][column_index]

                # Check for blank value
                if not current_value:
                    blank_count = blank_count + 1
                    break
                # Check for non-matching value
                elif initial_value != current_value:
                    break
                else:
                    value_count = value_count + 1

        # Check for corner-to-corner win
        if not winner:
            if  player == self.board[0][0] == self.board[1][1] == self.board[2][2]:
                winner = True
            elif player == self.board[0][2] == self.board[1][1] == self.board[2][0]:
                winner = True


        if winner:
            self.current_state = player + "_WON"
        elif blank_count == 0:
            self.current_state = "DRAW"

        return True

row_test = TicTacToe()
print(row_test.get_current_state())
row_test.make_move(0,0,"x")
row_test.make_move(0,1,"x")
row_test.make_move(0,2,"x")
print(row_test.get_current_state())
row_test.make_move(0,2,"x")

column_test = TicTacToe()
print(column_test.get_current_state())
column_test.make_move(0,1,"x")
column_test.make_move(1,1,"x")
column_test.make_move(2,1,"x")
print(column_test.get_current_state())
column_test.make_move(2,1,"x")

forward_slash_test = TicTacToe()
print(forward_slash_test.get_current_state())
forward_slash_test.make_move(2,0,"o")
forward_slash_test.make_move(1,1,"o")
forward_slash_test.make_move(0,2,"o")
print(forward_slash_test.get_current_state())
forward_slash_test.make_move(2,1,"o")

backward_slash_test = TicTacToe()
print(backward_slash_test.get_current_state())
backward_slash_test.make_move(0,2,"x")
backward_slash_test.make_move(1,1,"x")
backward_slash_test.make_move(2,0,"x")
print(backward_slash_test.get_current_state())
backward_slash_test.make_move(2,1,"x")

no_winner_test = TicTacToe()
print(no_winner_test.get_current_state())
no_winner_test.make_move(0,2,"x")
no_winner_test.make_move(1,1,"o")
no_winner_test.make_move(2,0,"x")
print(no_winner_test.get_current_state())
no_winner_test.make_move(2,1,"x")
print(no_winner_test.get_current_state())