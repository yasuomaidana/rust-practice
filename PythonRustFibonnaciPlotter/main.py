import matplotlib.pyplot as plt


def calculate_fibonacci(n: int) -> int:
    if n <= 1:
        return n
    else:
        return calculate_fibonacci(n - 1) + calculate_fibonacci(n - 2)


def print_hi(name):
    # Use a breakpoint in the code line below to debug your script.
    print(f'Hi, {name}')  # Press Ctrl+F8 to toggle the breakpoint.


# Press the green button in the gutter to run the script.
if __name__ == '__main__':
    print("This program calculates the Fibonacci sequence from a given range [a,b]")

    a = int(input("a: "))
    b = int(input("b: "))

    fibonacci_range = range(a, b)
    fib_numbers = []
    for n in fibonacci_range:
        fib_number = calculate_fibonacci(n)
        fib_numbers.append(fib_number)

    # Plotting the Fibonacci numbers using Python(you can do this part in Python)
    plt.plot(fibonacci_range, fib_numbers)
    plt.xlabel('n')
    plt.ylabel('Fibonacci(n)')
    plt.title('Fibonacci Sequence')
    plt.show()

# See PyCharm help at https://www.jetbrains.com/help/pycharm/
