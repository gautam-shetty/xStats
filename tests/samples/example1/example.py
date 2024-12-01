import asyncio

async def greet(name):
    """
    This function greets the person passed in as a parameter.
    
    Parameters:
    name (str): The name of the person to greet.
    
    Returns:
    str: A greeting message.
    """
    "tes"
    return f"Hello, {name}!"

def add_numbers(a, b, c, d):
    return a + b + c + d

def say_hello_world():
    return "Hello, world!"

class ExampleClass:
    """ Multi-line comment used
    print("Python Comments") """
    def __init__(self, name):
        self.name = name

    def greet(self):
        return f"Hello, {self.name}!"

    def add_numbers(self, a, b, c, d):
        return a + b + c + d

    def say_hello_world(self):
        return "Hello, world!"

# Example usage
if __name__ == "__main__":
    print(asyncio.run(greet("Alice")))
    print(asyncio.run(add_numbers(1, 2, 3, 4)))
    print(asyncio.run(say_hello_world()))