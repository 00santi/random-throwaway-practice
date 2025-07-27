import javax.swing.*;
import java.awt.*;

public class MyFrame extends JFrame{
    MyFrame() {
        this.setTitle("My first jframe");
        this.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        this.setResizable(false);
        this.setSize(480, 480);
        this.setVisible(true);

        this.getContentPane().setBackground(Color.GRAY);
    }
}
