package info.dmerej.gumball;

public class GumballMachine {
    private int count;
    private State state;

    public GumballMachine() {
        this.count = 0;
        this.state = State.SoldOut;
    }

    /*
      Note: this creates a Gumball machine directly in the desire state,
      but this by-passes all transitions rules

      You can use this method to test this class, or maybe to load it
      from a persistent database, but be careful
     */
    public static GumballMachine withState(State state, int count) {
        var res = new GumballMachine();
        res.state = state;
        res.count = count;
        return res;
    }

    public static GumballMachine withState(State state) {
        return GumballMachine.withState(state, 10);
    }


    public State getState() {
        return state;
    }

    public void fill(int gumballCount) {
        this.count = gumballCount;
        this.state = State.NoQuarter;
    }


    public void insertQuarter() {
        switch (this.state) {
            case HasQuarter -> {
                throw new IllegalStateException("You can't insert another quarter");
            }
            case SoldOut -> {
                throw new IllegalStateException("Machine is sold out");
            }
            case Sold -> {
                throw new IllegalStateException("We're already giving you a gumball");
            }
            case NoQuarter -> {
                this.state = State.HasQuarter;
            }
        }
    }

    public void ejectQuarter() {
        switch (this.state) {
            case HasQuarter -> {
                this.state = State.NoQuarter;
            }
            case SoldOut -> {
                throw new IllegalStateException("Machine is sold out");
            }
            case Sold -> {
                throw new IllegalStateException("We're already giving you a gumball");
            }
            case NoQuarter -> {
                throw new IllegalStateException("No quarter to eject");
            }
        }
    }

    public void turnCrank() {
        switch (this.state) {
            case HasQuarter -> {
                this.state = State.Sold;
                dispense();
            }
            case SoldOut -> {
                throw new IllegalStateException("Machine is sold out");
            }
            case Sold -> {
                throw new IllegalStateException("We're already giving you a gumball");
            }
            case NoQuarter -> {
                // nothing to do
            }
        }
    }

    private void dispense() {
        if (state != State.Sold) {
            throw new AssertionError("Should never happen ...");
        }

        count--;
        if (count == 0) {
            this.state = State.SoldOut;
        } else {
            this.state = State.NoQuarter;
        }
    }

    public int getCount() {
        return count;
    }
}
