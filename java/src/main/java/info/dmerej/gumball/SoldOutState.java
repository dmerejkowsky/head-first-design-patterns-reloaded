package info.dmerej.gumball;

public class SoldOutState implements State {
    private final GumballMachine gumballMachine;

    public SoldOutState(GumballMachine gumballMachine) {
        this.gumballMachine = gumballMachine;
    }

    @Override
    public void insertQuarter() {
        throw new IllegalStateException("sold out");
    }

    @Override
    public void ejectQuarter() {
        throw new IllegalStateException("sold out");
    }

    @Override
    public void turnCrank() {
        // nothing to do
    }

    @Override
    public void dispense() {
        throw new IllegalStateException("sold out");
    }
}
