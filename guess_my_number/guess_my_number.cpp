#include <iostream>
#include <stdlib.h>
#include <string>

using namespace std;

int caught = 0;
int tries = 0;

enum Location
{
	HIGH,
	LOW,
};

void exception_caught()
{
	caught = 1;
	cout << "Guess Again\n";
}

int getGuess()
{
	string tmp;
	cin >> tmp;
	tries++;
	return stoi(tmp);
}

void print_message(Location high_low)
{
	if (caught)
	{
		caught = 0;
		return;
	}
	if (high_low == LOW)
	{
		cout << "Too LOW\n";
	}
	else if (high_low == HIGH)
	{
		cout << "Too HIGH\n";
	}
}

bool Finished(int &guess, int number, int &tries)
{
	if (guess == number)
	{
		return true;
	}
	else if (guess < number)
	{

		print_message(LOW);
		guess = getGuess();
	}
	else if (guess > number)
	{

		print_message(HIGH);
		guess = getGuess();
	}
	return false;
}

void game(int &guess, int number, int &tries)
{
	try
	{
		while (!Finished(guess, number, tries))
		{
		}
		cout << "Congratulations, you found it in " << tries << " tries!" << endl;
	}
	catch (const exception &e)
	{
		exception_caught();
		game(guess, number, tries);
	}
}

int main()
{

	int number = rand() %500;
	string guess;
	int guess_int;
	try
	{
		cout << "Guess my number!" << endl;
		guess_int = getGuess();
	}
	catch (const exception &e)
	{
		exception_caught();
	}
	game(guess_int, number, tries);

	return 0;
}
