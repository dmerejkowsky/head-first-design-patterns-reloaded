package info.dmerej.menus;

import java.util.List;

public class App {
    public static void main(String[] args) {
        var dinerMenu = new DinerMenu();
        var pancakeHouseMenu = new PancakeHouseMenu();
        var menus = List.of(dinerMenu, pancakeHouseMenu);
        var waitress = new Waitress(menus);
        waitress.printMenus();
    }
}