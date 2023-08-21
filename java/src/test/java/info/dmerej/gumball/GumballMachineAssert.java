package info.dmerej.gumball;

import org.assertj.core.api.AbstractAssert;

import static org.assertj.core.api.Assertions.assertThat;

public class GumballMachineAssert extends AbstractAssert<GumballMachineAssert, GumballMachine> {
    public GumballMachineAssert(GumballMachine actual) {
        super(actual, GumballMachineAssert.class);
    }

    public GumballMachineAssert isNotSoldOut() {
        isNotNull();
        if (!(actual.getState() instanceof SoldOutState)) {
            failWithMessage("Machine should not be sold out");
        }
        return this;
    }

    public GumballMachineAssert hasState(State expectedState) {
        isNotNull();
        var actualState = actual.getState();
        var expectedClassName = expectedState.getClass().getSimpleName();
        var actualClassName = actualState.getClass().getSimpleName();
        if (actualClassName != expectedClassName) {
            failWithMessage("Expected state to be %s but was %s", expectedClassName, actualClassName);
        }
        return this;
    }

    public void hasCount(int expectedCount) {
        assertThat(actual.getCount()).isEqualTo(expectedCount);
    }
}
