##
## EtienneP-26 PROJECT, 2025
## NyxoCode
## File description:
## Makefile
##

NAME = ncsh

all: $(NAME)

$(NAME):
	cd NyxoCode && cargo build --release
	cp NyxoCode/target/release/NyxoCode $(NAME)
	clear
	@echo "NyxoCode built successfully"

clean:
	cd NyxoCode && cargo clean

fclean: clean
	rm -f $(NAME)

re: fclean all

run: all
	./$(NAME)
