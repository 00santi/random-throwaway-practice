import javax.swing.*;
import java.awt.*;

public class Main {
    public static void main(String[] args) {
        var frame = new JFrame();
        frame.setTitle("Second jframe");
        frame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
        frame.setSize(480, 480);

        var label = new JLabel();


        var img = new ImageIcon(Main.class.getResource("/cat.png"));
        var img2 = img.getImage();
        img2 = img2.getScaledInstance(200, 200, Image.SCALE_SMOOTH);
        img = new ImageIcon(img2);
        label.setIcon(img);

        label.setText("First JLabel");
        label.setHorizontalTextPosition(JLabel.CENTER);
        label.setVerticalTextPosition(JLabel.TOP);
        label.setForeground(Color.RED);


        label.setBackground(Color.BLACK);
        label.setOpaque(true);


        var border = BorderFactory.createLineBorder(Color.RED, 3);
        label.setBorder(border);


        label.setVerticalAlignment(JLabel.CENTER);
        label.setHorizontalAlignment(JLabel.CENTER);
        //label.setBounds(0, 0, 250, 250);

        //frame.setLayout(null);
        frame.add(label);
        frame.setVisible(true);
    }
}
