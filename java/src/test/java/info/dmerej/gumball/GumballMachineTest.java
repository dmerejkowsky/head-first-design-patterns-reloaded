package info.dmerej.gumball;

import org.junit.jupiter.api.Test;

import java.util.Arrays;

import static info.dmerej.gumball.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatThrownBy;


class GumballMachineTest {
    @Test
    void going_from_no_quarter_to_has_quarter() {
        var gumballMachine = GumballMachine.withState(State.NoQuarter, 10);
        gumballMachine.insertQuarter();
        assertThat(gumballMachine).hasState(State.HasQuarter);
    }

    @Test
    void cannot_insert_quarter_twice() {
        var gumballMachine = GumballMachine.withState(State.HasQuarter, 10);
        assertThatThrownBy(() -> gumballMachine.insertQuarter())
            .hasMessageContaining("can't insert another quarter");
    }

    @Test
    void cannot_insert_quarter_when_sold_od() {
        var gumballMachine = GumballMachine.withState(State.SoldOut, 0);
        assertThatThrownBy(() -> gumballMachine.insertQuarter())
            .hasMessageContaining("sold out");
    }

    @Test
    void cannot_insert_quarter_while_gumball_is_sold() {
        var gumballMachine = GumballMachine.withState(State.Sold, 10);
        assertThatThrownBy(() -> gumballMachine.insertQuarter())
            .hasMessageContaining("already giving you a gumball");
    }

    @Test
    void can_eject_a_quarter_if_one_is_present() {
        var gumballMachine = GumballMachine.withState(State.HasQuarter, 10);
        gumballMachine.ejectQuarter();
        assertThat(gumballMachine).hasState(State.NoQuarter);
        var allOtherStates = Arrays.stream(State.values()).filter(x -> x != State.HasQuarter);
        allOtherStates.forEach(state -> {
            var g = GumballMachine.withState(state, 10);
            assertThatThrownBy(() -> g.ejectQuarter()).isInstanceOf(IllegalStateException.class);
        });
    }

    @Test
    void turning_crank_when_machine_is_filled_and_has_a_quarter_dispenses_a_gumball() {
        var gumballMachine = GumballMachine.withState(State.HasQuarter, 10);
        gumballMachine.turnCrank();
        assertThat(gumballMachine).hasState(State.NoQuarter);
        assertThat(gumballMachine).hasCount(9);
    }

    @Test
    void turning_crank_when_no_quarter_is_there_does_nothing() {
        var gumballMachine = GumballMachine.withState(State.NoQuarter, 10);
        gumballMachine.turnCrank();
        assertThat(gumballMachine).hasState(State.NoQuarter);
    }

    @Test
    void turning_crank_rendering_machine_sold_out() {
        var gumballMachine = GumballMachine.withState(State.HasQuarter, 1);
        gumballMachine.turnCrank();
        assertThat(gumballMachine).hasCount(0);
        assertThat(gumballMachine).hasState(State.SoldOut);
    }
}