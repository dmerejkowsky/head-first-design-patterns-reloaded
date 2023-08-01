package info.dmerej.menus;

public class Waitress {
    private final MenuComponent allMenus;

    public Waitress(MenuComponent allMenus) {
        this.allMenus = allMenus;
    }

    public void printMenus() {
        allMenus.print();
    }

}
