namespace KitX_Installer_for_Windows_in.NET_Framework
{
    partial class AskLanguage
    {
        /// <summary>
        /// Required designer variable.
        /// </summary>
        private System.ComponentModel.IContainer components = null;

        /// <summary>
        /// Clean up any resources being used.
        /// </summary>
        /// <param name="disposing">true if managed resources should be disposed; otherwise, false.</param>
        protected override void Dispose(bool disposing)
        {
            if (disposing && (components != null))
            {
                components.Dispose();
            }
            base.Dispose(disposing);
        }

        #region Windows Form Designer generated code

        /// <summary>
        /// Required method for Designer support - do not modify
        /// the contents of this method with the code editor.
        /// </summary>
        private void InitializeComponent()
        {
            System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(AskLanguage));
            this.comboBox_languageSelector = new System.Windows.Forms.ComboBox();
            this.button_select = new System.Windows.Forms.Button();
            this.SuspendLayout();
            // 
            // comboBox_languageSelector
            // 
            this.comboBox_languageSelector.Font = new System.Drawing.Font("微软雅黑", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(134)));
            this.comboBox_languageSelector.FormattingEnabled = true;
            this.comboBox_languageSelector.Items.AddRange(new object[] {
            "English",
            "简体中文"});
            this.comboBox_languageSelector.Location = new System.Drawing.Point(12, 12);
            this.comboBox_languageSelector.Name = "comboBox_languageSelector";
            this.comboBox_languageSelector.Size = new System.Drawing.Size(260, 29);
            this.comboBox_languageSelector.TabIndex = 0;
            // 
            // button_select
            // 
            this.button_select.Font = new System.Drawing.Font("微软雅黑", 10F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(134)));
            this.button_select.Location = new System.Drawing.Point(172, 59);
            this.button_select.Name = "button_select";
            this.button_select.Size = new System.Drawing.Size(100, 40);
            this.button_select.TabIndex = 1;
            this.button_select.Text = "Select";
            this.button_select.UseVisualStyleBackColor = true;
            // 
            // AskLanguage
            // 
            this.AutoScaleDimensions = new System.Drawing.SizeF(6F, 12F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.ClientSize = new System.Drawing.Size(284, 111);
            this.Controls.Add(this.button_select);
            this.Controls.Add(this.comboBox_languageSelector);
            this.Icon = ((System.Drawing.Icon)(resources.GetObject("$this.Icon")));
            this.MaximizeBox = false;
            this.MinimizeBox = false;
            this.Name = "AskLanguage";
            this.SizeGripStyle = System.Windows.Forms.SizeGripStyle.Hide;
            this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
            this.Text = "Select Language";
            this.ResumeLayout(false);

        }

        #endregion

        private System.Windows.Forms.ComboBox comboBox_languageSelector;
        private System.Windows.Forms.Button button_select;
    }
}
