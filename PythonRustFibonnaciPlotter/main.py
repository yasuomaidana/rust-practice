import matplotlib.pyplot as plt
from fibonacci_lib import calculate_fibonacci

# Press the green button in the gutter to run the script.
if __name__ == '__main__':
    print("This program calculates the Fibonacci sequence from a given range [a,b]")

    a = int(input("a: "))
    b = int(input("b: "))

    fibonacci_range = range(a, b)
    fib_numbers = []
    for ni in fibonacci_range:
        fib_number = calculate_fibonacci(ni)
        fib_numbers.append(fib_number)

    # Plotting the Fibonacci numbers using Python(you can do this part in Python)
    plt.plot(fibonacci_range, fib_numbers)
    plt.xlabel('n')
    plt.ylabel('Fibonacci(n)')
    plt.title('Fibonacci Sequence')
    plt.show()
