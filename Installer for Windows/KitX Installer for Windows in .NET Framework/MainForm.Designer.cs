namespace KitX_Installer_for_Windows_in.NET_Framework
{
    partial class MainForm
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
            System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(MainForm));
            this.Btn_BeginInstall = new System.Windows.Forms.Button();
            this.TextBox_InstallPath = new System.Windows.Forms.TextBox();
            this.ProgressBar_Installing = new System.Windows.Forms.ProgressBar();
            this.Label_Tip = new System.Windows.Forms.Label();
            this.checkBox_startAfterInstall = new System.Windows.Forms.CheckBox();
            this.checkBox_desktopShortcut = new System.Windows.Forms.CheckBox();
            this.checkBox_startUpMenuShortCut = new System.Windows.Forms.CheckBox();
            this.checkBox_addPath = new System.Windows.Forms.CheckBox();
            this.SuspendLayout();
            // 
            // Btn_BeginInstall
            // 
            this.Btn_BeginInstall.DialogResult = System.Windows.Forms.DialogResult.Cancel;
            this.Btn_BeginInstall.Font = new System.Drawing.Font("微软雅黑", 14.25F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(134)));
            this.Btn_BeginInstall.Location = new System.Drawing.Point(580, 480);
            this.Btn_BeginInstall.Name = "Btn_BeginInstall";
            this.Btn_BeginInstall.Size = new System.Drawing.Size(180, 50);
            this.Btn_BeginInstall.TabIndex = 0;
            this.Btn_BeginInstall.Text = "安装";
            this.Btn_BeginInstall.UseVisualStyleBackColor = true;
            this.Btn_BeginInstall.Click += new System.EventHandler(this.Btn_BeginInstall_Click);
            // 
            // TextBox_InstallPath
            // 
            this.TextBox_InstallPath.Font = new System.Drawing.Font("微软雅黑", 12F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(134)));
            this.TextBox_InstallPath.Location = new System.Drawing.Point(200, 420);
            this.TextBox_InstallPath.Name = "TextBox_InstallPath";
            this.TextBox_InstallPath.Size = new System.Drawing.Size(400, 29);
            this.TextBox_InstallPath.TabIndex = 1;
            this.TextBox_InstallPath.Text = "C:\\Program Files\\Crequency\\KitX";
            // 
            // ProgressBar_Installing
            // 
            this.ProgressBar_Installing.Dock = System.Windows.Forms.DockStyle.Bottom;
            this.ProgressBar_Installing.Location = new System.Drawing.Point(0, 556);
            this.ProgressBar_Installing.Name = "ProgressBar_Installing";
            this.ProgressBar_Installing.Size = new System.Drawing.Size(784, 5);
            this.ProgressBar_Installing.TabIndex = 2;
            this.ProgressBar_Installing.Visible = false;
            // 
            // Label_Tip
            // 
            this.Label_Tip.AutoSize = true;
            this.Label_Tip.Font = new System.Drawing.Font("微软雅黑", 9F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(134)));
            this.Label_Tip.Location = new System.Drawing.Point(5, 537);
            this.Label_Tip.Name = "Label_Tip";
            this.Label_Tip.Size = new System.Drawing.Size(86, 17);
            this.Label_Tip.TabIndex = 3;
            this.Label_Tip.Text = "等待用户操作..";
            // 
            // checkBox_startAfterInstall
            // 
            this.checkBox_startAfterInstall.AutoSize = true;
            this.checkBox_startAfterInstall.Checked = true;
            this.checkBox_startAfterInstall.CheckState = System.Windows.Forms.CheckState.Checked;
            this.checkBox_startAfterInstall.Font = new System.Drawing.Font("微软雅黑", 9F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(134)));
            this.checkBox_startAfterInstall.Location = new System.Drawing.Point(26, 475);
            this.checkBox_startAfterInstall.Name = "checkBox_startAfterInstall";
            this.checkBox_startAfterInstall.Size = new System.Drawing.Size(114, 21);
            this.checkBox_startAfterInstall.TabIndex = 4;
            this.checkBox_startAfterInstall.Text = "安装后运行 KitX";
            this.checkBox_startAfterInstall.UseVisualStyleBackColor = true;
            // 
            // checkBox_desktopShortcut
            // 
            this.checkBox_desktopShortcut.AutoSize = true;
            this.checkBox_desktopShortcut.Checked = true;
            this.checkBox_desktopShortcut.CheckState = System.Windows.Forms.CheckState.Checked;
            this.checkBox_desktopShortcut.Font = new System.Drawing.Font("微软雅黑", 9F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(134)));
            this.checkBox_desktopShortcut.Location = new System.Drawing.Point(282, 475);
            this.checkBox_desktopShortcut.Name = "checkBox_desktopShortcut";
            this.checkBox_desktopShortcut.Size = new System.Drawing.Size(123, 21);
            this.checkBox_desktopShortcut.TabIndex = 5;
            this.checkBox_desktopShortcut.Text = "生成桌面快捷方式";
            this.checkBox_desktopShortcut.UseVisualStyleBackColor = true;
            // 
            // checkBox_startUpMenuShortCut
            // 
            this.checkBox_startUpMenuShortCut.AutoSize = true;
            this.checkBox_startUpMenuShortCut.Checked = true;
            this.checkBox_startUpMenuShortCut.CheckState = System.Windows.Forms.CheckState.Checked;
            this.checkBox_startUpMenuShortCut.Font = new System.Drawing.Font("微软雅黑", 9F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(134)));
            this.checkBox_startUpMenuShortCut.Location = new System.Drawing.Point(282, 502);
            this.checkBox_startUpMenuShortCut.Name = "checkBox_startUpMenuShortCut";
            this.checkBox_startUpMenuShortCut.Size = new System.Drawing.Size(147, 21);
            this.checkBox_startUpMenuShortCut.TabIndex = 6;
            this.checkBox_startUpMenuShortCut.Text = "生成开始菜单快捷方式";
            this.checkBox_startUpMenuShortCut.UseVisualStyleBackColor = true;
            // 
            // checkBox_addPath
            // 
            this.checkBox_addPath.AutoSize = true;
            this.checkBox_addPath.Checked = true;
            this.checkBox_addPath.CheckState = System.Windows.Forms.CheckState.Checked;
            this.checkBox_addPath.Font = new System.Drawing.Font("微软雅黑", 9F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(134)));
            this.checkBox_addPath.Location = new System.Drawing.Point(26, 502);
            this.checkBox_addPath.Name = "checkBox_addPath";
            this.checkBox_addPath.Size = new System.Drawing.Size(92, 21);
            this.checkBox_addPath.TabIndex = 7;
            this.checkBox_addPath.Text = "添加到 Path";
            this.checkBox_addPath.UseVisualStyleBackColor = true;
            // 
            // MainForm
            // 
            this.AcceptButton = this.Btn_BeginInstall;
            this.AutoScaleDimensions = new System.Drawing.SizeF(6F, 12F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.BackColor = System.Drawing.Color.White;
            this.ClientSize = new System.Drawing.Size(784, 561);
            this.Controls.Add(this.checkBox_addPath);
            this.Controls.Add(this.checkBox_startUpMenuShortCut);
            this.Controls.Add(this.checkBox_desktopShortcut);
            this.Controls.Add(this.checkBox_startAfterInstall);
            this.Controls.Add(this.Label_Tip);
            this.Controls.Add(this.ProgressBar_Installing);
            this.Controls.Add(this.TextBox_InstallPath);
            this.Controls.Add(this.Btn_BeginInstall);
            this.FormBorderStyle = System.Windows.Forms.FormBorderStyle.FixedSingle;
            this.Icon = ((System.Drawing.Icon)(resources.GetObject("$this.Icon")));
            this.MaximizeBox = false;
            this.Name = "MainForm";
            this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
            this.Text = "KitX Installer | KitX 安装向导";
            this.ResumeLayout(false);
            this.PerformLayout();

        }

        #endregion

        private System.Windows.Forms.Button Btn_BeginInstall;
        private System.Windows.Forms.TextBox TextBox_InstallPath;
        private System.Windows.Forms.ProgressBar ProgressBar_Installing;
        private System.Windows.Forms.Label Label_Tip;
        private System.Windows.Forms.CheckBox checkBox_startAfterInstall;
        private System.Windows.Forms.CheckBox checkBox_desktopShortcut;
        private System.Windows.Forms.CheckBox checkBox_startUpMenuShortCut;
        private System.Windows.Forms.CheckBox checkBox_addPath;
    }
}