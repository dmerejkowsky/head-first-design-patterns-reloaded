package info.dmerej.menus;

import java.util.Arrays;
import java.util.stream.Stream;

public class DinerMenu implements Menu {
    private final int NUMBER_OF_ITEMS = 6;
    private final MenuItem[] items;
    private int numberOfItems = 0;

    DinerMenu() {
        items = new MenuItem[NUMBER_OF_ITEMS];
        addItem("BLT", 3);
        addItem("Hot Dog", 2);
    }

    private void addItem(String name, int price) {
        var item = new MenuItem(name, price);
        items[numberOfItems] = item;
        numberOfItems++;
    }

    public Stream<MenuItem> getItems() {
        return Arrays.stream(items).limit(numberOfItems);
    }
}
