using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace KitX_Installer_for_Windows_in.NET_Framework
{
    public partial class AskLanguage : Form
    {
        public AskLanguage()
        {
            InitializeComponent();

            comboBox_languageSelector.SelectedIndex = 0;
        }

        public void OnSelect(Action<int> action)
        {
            button_select.Click += (sender, e) =>
            {
                if (comboBox_languageSelector.SelectedIndex >= 0)
                {
                    action.Invoke(comboBox_languageSelector.SelectedIndex);
                    Close();
                }
            };
        }
    }
}
