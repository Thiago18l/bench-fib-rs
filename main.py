import fibonacci


def fibonacci_py(n) -> int:
    if n == 0:
        return 0
    if n == 1:
        return 1
    return fibonacci_py(n - 1) + fibonacci_py(n - 2)


def test_pure_python(benchmark):
    benchmark(fibonacci_py, 10)


def test_rust(benchmark):
    benchmark(fibonacci.fibonacci, 10)


if __name__ == '__main__':
    print(fibonacci.fibonacci(10))
    print(fibonacci_py(10))
