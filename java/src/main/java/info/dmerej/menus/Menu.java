package info.dmerej.menus;

import java.util.stream.Stream;

public interface Menu {
    Stream<MenuItem> getItems();
}
