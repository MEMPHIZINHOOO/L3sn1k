import java.awt.BorderLayout;
import java.awt.Dimension;
import java.awt.EventQueue;
import java.io.File;

import javax.swing.Box;
import javax.swing.BoxLayout;
import javax.swing.ButtonGroup;
import javax.swing.JButton;
import javax.swing.JFileChooser;
import javax.swing.JFrame;
import javax.swing.JLabel;
import javax.swing.JPanel;
import javax.swing.JRadioButton;
import javax.swing.JTextField;
import javax.swing.border.EmptyBorder;

import App;


public class Init {


    public static void main(String[] args) {
        EventQueue.invokeLater(() -> {
            JFrame frame = new JFrame("L3snki v0.1");
            frame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);
            frame.setPreferredSize(new Dimension(600, 300));
            frame.setLayout(new BorderLayout());

            // main Panel is responsible for the three buttons and the continue field
            JPanel mainPanel = new JPanel();
            mainPanel.setBorder(new EmptyBorder(15, 15, 15, 15));
            mainPanel.setLayout(new BoxLayout(mainPanel, BoxLayout.Y_AXIS));

            // creates the three buttons responsible for choosing files
            JRadioButton b1 = new JRadioButton("Temporary Project");
            JRadioButton b2 = new JRadioButton("New Project on Disk");
            JRadioButton b3 = new JRadioButton("Open Existing Project");

            // default parameter sent to Projects
            String selectedOption = "Temporary Project";
            ButtonGroup group = new ButtonGroup();
            group.add(b1);
            group.add(b2);
            group.add(b3);
    
            b1.setSelected(true);

            b1.setAlignmentX(JPanel.LEFT_ALIGNMENT);
            b2.setAlignmentX(JPanel.LEFT_ALIGNMENT);
            b3.setAlignmentX(JPanel.LEFT_ALIGNMENT);

            // new project Panel
            
            JPanel newProjectPanel = new JPanel();
            newProjectPanel.setLayout(new BoxLayout(newProjectPanel, BoxLayout.X_AXIS));
            newProjectPanel.setAlignmentX(JPanel.LEFT_ALIGNMENT);
        
            JLabel newProjectLabel = new JLabel("New project path:");
            JTextField newProjectField = new JTextField();
            JButton newProjectBrowse = new JButton("Browse...");

            newProjectPanel.add(newProjectLabel);
            newProjectPanel.add(Box.createHorizontalStrut(10));
            newProjectPanel.add(newProjectField);
            newProjectPanel.add(Box.createHorizontalStrut(10));
            newProjectPanel.add(newProjectBrowse);

            // open existing Project File Panel
            JPanel openProjectPanel = new JPanel();
            openProjectPanel.setLayout(new BoxLayout(openProjectPanel, BoxLayout.X_AXIS));
            openProjectPanel.setAlignmentX(JPanel.LEFT_ALIGNMENT);

            JLabel openProjectLabel = new JLabel("Existing project file:");
            JTextField openProjectField = new JTextField();
            JButton openProjectBrowse = new JButton("Browse...");

            openProjectPanel.add(openProjectLabel);
            openProjectPanel.add(Box.createHorizontalStrut(10));
            openProjectPanel.add(openProjectField);
            openProjectPanel.add(Box.createHorizontalStrut(10));
            openProjectPanel.add(openProjectBrowse);

            newProjectField.setMaximumSize(new Dimension(Integer.MAX_VALUE, 30));
            openProjectField.setMaximumSize(new Dimension(Integer.MAX_VALUE, 30));

            newProjectPanel.setVisible(false);
            openProjectPanel.setVisible(false);

            // action listeners
            b1.addActionListener(e -> {
                newProjectPanel.setVisible(false);
                openProjectPanel.setVisible(false);
                //probably not efficient
                JRadioButton radioButton = (JRadioButton)e.getSource();
                selectedOption = radioButton.getText();

                frame.pack();
            });

            b2.addActionListener(e -> {
                newProjectPanel.setVisible(true);
                openProjectPanel.setVisible(false);
                //probably not efficient
                JRadioButton radioButton = (JRadioButton)e.getSource();
                selectedOption = radioButton.getText();
                frame.pack();
            });

            b3.addActionListener(e -> {
                newProjectPanel.setVisible(false);
                openProjectPanel.setVisible(true);
                frame.pack();
                //probably not efficient
                JRadioButton radioButton = (JRadioButton)e.getSource();
                selectedOption = radioButton.getText();
            });
            
            newProjectBrowse.addActionListener(e -> {
                JFileChooser chooser = new JFileChooser();
                chooser.setDialogTitle("Choose where to create the project");
                int result = chooser.showSaveDialog(frame);

                if (result == JFileChooser.APPROVE_OPTION) {
                    File selectedFile = chooser.getSelectedFile();
                    newProjectField.setText(selectedFile.getAbsolutePath());
                }
            });

            openProjectBrowse.addActionListener(e -> {
                JFileChooser chooser = new JFileChooser();
                chooser.setDialogTitle("Open existing project");
                int result = chooser.showOpenDialog(frame);

                if (result == JFileChooser.APPROVE_OPTION) {
                    File selectedFile = chooser.getSelectedFile();
                    openProjectField.setText(selectedFile.getAbsolutePath());
                }
            });
            JButton b4 = new JButton("Continue");

            b4.setAlignmentY(JPanel.BOTTOM_ALIGNMENT);

            mainPanel.add(b1);
            mainPanel.add(Box.createVerticalStrut(10));
            mainPanel.add(b2);
            mainPanel.add(Box.createVerticalStrut(10));
            mainPanel.add(newProjectPanel);
            mainPanel.add(Box.createVerticalStrut(10));
            mainPanel.add(b3);
            mainPanel.add(Box.createVerticalStrut(10));
            mainPanel.add(openProjectPanel);
            mainPanel.add(b4);
            
            frame.add(mainPanel, BorderLayout.CENTER);

            frame.pack();
            frame.setLocationRelativeTo(null);
            frame.setVisible(true);

            // shifts to the next frame    
                b4.addActionListener(e -> {
                newProjectPanel.setVisible(false);
                openProjectPanel.setVisible(false);
                
                
                frame.dispose();
                App.App.start(selectedOption, newProjectBrowse, openProjectBrowse);                                
            });

        });
    }
}
