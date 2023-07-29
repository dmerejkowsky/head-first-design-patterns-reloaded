package info.dmerej.barrista;

public class Tea extends HotBeverage {
    public void prepareRecipe() {
        boilWater();
        steepTeaBag();
        pourInCup();
        addLemon();
    }

    private void steepTeaBag() {
        System.out.println("Steeping the tea");
    }
    
    private void addLemon() {
        System.out.println("Adding sugar lemon");
    }

}
