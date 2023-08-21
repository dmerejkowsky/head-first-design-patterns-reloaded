package info.dmerej.gumball;

public class GumballMachine {
    private final State noQuarterState;
    private final State hasQuarterState;
    private final State soldOutState;
    private final State soldState;

    private State state;

    private int count;

    public GumballMachine(int count) {
        this.count = count;
        hasQuarterState = new HasQuarterState(this);
        noQuarterState = new NoQuarterState(this);
        soldOutState = new SoldOutState(this);
        soldState = new SoldState(this);
        state = soldOutState;
    }

    public State getNoQuarterState() {
        return noQuarterState;
    }

    public State getHasQuarterState() {
        return hasQuarterState;
    }

    public int getCount() {
        return count;
    }

    public void setCount(int count) {
        this.count = count;
    }

    public void fill(int gumballCount) {
        count = gumballCount;
        state = new NoQuarterState(this);
    }

    public void insertQuarter() {
        this.state.insertQuarter();
    }

    public void ejectQuarter() {
        this.state.ejectQuarter();
    }

    public void turnCrank() {
        state.turnCrank();
        state.dispense();
    }

    public State getSoldOutState() {
        return soldOutState;
    }

    public State getSoldState() {
        return soldState;
    }

    public State getState() {
        return state;
    }

    public void setState(State state) {
        this.state = state;
    }

    public void releaseBall() {
        count--;
    }
}
