package info.dmerej.gumball;

public class Assertions {
    public static GumballMachineAssert assertThat(GumballMachine actual) {
        return new GumballMachineAssert(actual);
    }
}
