package info.dmerej.gumball;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;

import static info.dmerej.gumball.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;


class GumballMachineTest {
    private GumballMachine gumballMachine;

    @BeforeEach
    void setUp() {
        gumballMachine = new GumballMachine(10);
    }

    @Test
    void going_from_no_quarter_to_has_quarter() {
        gumballMachine.setState(gumballMachine.getNoQuarterState());
        gumballMachine.insertQuarter();
        assertThat(gumballMachine).hasState(gumballMachine.getHasQuarterState());
    }

    @Test
    void cannot_insert_quarter_twice() {
        gumballMachine.setState(gumballMachine.getHasQuarterState());
        assertThatThrownBy(() -> gumballMachine.insertQuarter())
            .hasMessageContaining("can't insert another quarter");
    }

    @Test
    void cannot_insert_quarter_when_sold_od() {
        gumballMachine.setCount(0);
        gumballMachine.setState(gumballMachine.getSoldOutState());
        assertThatThrownBy(() -> gumballMachine.insertQuarter())
            .hasMessageContaining("sold out");
    }

    @Test
    void cannot_insert_quarter_while_gumball_is_sold() {
        gumballMachine.setState(gumballMachine.getSoldState());
        assertThatThrownBy(() -> gumballMachine.insertQuarter())
            .hasMessageContaining("already giving you a gumball");
    }

    @Test
    void can_eject_a_quarter_if_one_is_present() {
        gumballMachine.setState(gumballMachine.getHasQuarterState());
        gumballMachine.ejectQuarter();
        assertThat(gumballMachine).hasState(gumballMachine.getNoQuarterState());
        /*
        var allOtherStates = Arrays.stream(GumballMachineState.values()).filter(x -> x != GumballMachineState.HasQuarter);
        allOtherStates.forEach(state -> {
            var g = GumballMachine.withState(state, 10);
            assertThatThrownBy(() -> g.ejectQuarter()).isInstanceOf(IllegalStateException.class);
        });
         */
    }

    @Test
    void turning_crank_when_machine_is_filled_and_has_a_quarter_dispenses_a_gumball() {
        gumballMachine.setState(gumballMachine.getHasQuarterState());
        gumballMachine.turnCrank();
        assertThat(gumballMachine).hasState(gumballMachine.getNoQuarterState());
        assertThat(gumballMachine).hasCount(9);
    }

    @Test
    void turning_crank_when_no_quarter_is_there_does_nothing() {
        gumballMachine.setState(gumballMachine.getNoQuarterState());
        gumballMachine.turnCrank();
        assertThat(gumballMachine).hasState(gumballMachine.getNoQuarterState());
    }

    @Test
    void turning_crank_rendering_machine_sold_out() {
        gumballMachine.setCount(1);
        gumballMachine.setState(gumballMachine.getHasQuarterState());
        gumballMachine.turnCrank();
        assertThat(gumballMachine).hasCount(0);
        assertThat(gumballMachine).hasState(gumballMachine.getSoldOutState());
    }
}