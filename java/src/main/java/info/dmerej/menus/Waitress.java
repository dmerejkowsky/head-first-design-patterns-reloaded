package info.dmerej.menus;

import java.util.List;

public class Waitress {
    private final List<Menu> menus;

    public Waitress(List<Menu> menus) {
        this.menus = menus;
    }

    public void printMenus() {
        for (var menu : menus) {
            printMenu(menu);
        }
    }

    private void printMenu(Menu menu) {
        menu.getItems().forEach(item -> {
            System.out.println(item);
        });
    }
}
