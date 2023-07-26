package info.dmerej.coffeeshop;

import org.junit.jupiter.api.Test;

import static org.assertj.core.api.Assertions.assertThat;


class CoffeeShopTest {

    @Test
    void dark_roast_without_milk() {
        var beverage = new DarkRoast();
        assertThat(beverage.cost()).isEqualTo(10);
    }

    @Test
    void dark_roast_with_milk() {
        var darkRoast = new DarkRoast();
        var beverage = new Milk(darkRoast);
        assertThat(beverage.cost()).isEqualTo(12);
        assertThat(beverage.getDescription()).isEqualTo("dark roast with milk");
    }

    @Test
    void whipped_dark_roast_with_milk() {
        var darkRoast = new DarkRoast();
        var darkRostWithMilk = new Milk(darkRoast);
        var beverage = new Whip(darkRostWithMilk);
        assertThat(beverage.cost()).isEqualTo(13);
        assertThat(beverage.getDescription()).isEqualTo("whipped dark roast with milk");
    }

    @Test
    void tall_dark_roast() {
        var beverage = new DarkRoast();
        beverage.setSize(2);
        assertThat(beverage.cost()).isEqualTo(20);
    }

    @Test
    void tall_whipped_dark_roast() {
        var darkRoast = new DarkRoast();
        darkRoast.setSize(2);
        var beverage = new Whip(darkRoast);
        assertThat(beverage.cost()).isEqualTo(21);
    }
}