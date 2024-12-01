public class Example {
    public static void main(String[] args) {
        System.out.println("Hello, World!");
    }

    public static int add(int a, int b) {
        // This is a comment
        return a + b;
    }

    public static int subtract(int a, int b) {
        /*
         * This is a multi-line comment
         */
        return a - b;
    }

    public static int multiply(int a, int b) {
        /**
         * This is a JavaDoc comment
         */
        return a * b;
    }
}