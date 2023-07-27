package info.dmerej.pizza;

public abstract class Pizza {
    private String crust;

    public String getCrust() {
        return crust;
    }

    public void setCrust(String crust) {
        this.crust = crust;
    }

    abstract void prepare();

    public void bake() {
        System.out.println("Baking");
    }

    public void cut() {
        System.out.println("Cutting");
    }

    public void box() {
        System.out.println("Putting in box");
    }

    public abstract String name();
}
