#![allow(non_snake_case)]
#![allow(dead_code)]

use rust_vcl::fns::*;
use rust_vcl::messages::*;
use rust_vcl::types::*;
use rust_vcl::vcl::*;
use std::mem::size_of;

#[derive(VclForm)]
pub struct TMainForm {
    btn: TButton,
    btn2: TButton,
    btn3: TButton,
    btnOpenDialog: TButton,
    btnColorDialog: TButton,
    dlgOpen: TOpenDialog,
    dlgColor: TColorDialog,
    edit1: TEdit,
    memo1: TMemo,
    pub btnOpenForm2: TButton,
    pub btnOpenForm3: TButton,
    mainMenu: TMainMenu,
    popupMenu: TPopupMenu,
    tv1: TTreeView,
    pb1: TPaintBox,
    listBox1: TListBox,
    trayicon: TTrayIcon,
    pub form: TForm, // 固定名form, 放最后，前面引用完后，后面move到form。
}

impl TMainForm {
    pub fn new() -> Self {
        let form = Application.CreateForm();
        return Self {
            btn: NewObject!(TButton, form),
            btn2: NewObject!(TButton, form),
            btn3: NewObject!(TButton, form),
            btnOpenDialog: NewObject!(TButton, form),
            btnColorDialog: NewObject!(TButton, form),
            dlgOpen: NewObject!(TOpenDialog, form),
            dlgColor: NewObject!(TColorDialog, form),
            edit1: NewObject!(TEdit, form),
            memo1: NewObject!(TMemo, form),
            btnOpenForm2: NewObject!(TButton, form),
            btnOpenForm3: NewObject!(TButton, form),
            // menu
            mainMenu: NewObject!(TMainMenu, form),
            popupMenu: NewObject!(TPopupMenu, form),
            tv1: NewObject!(TTreeView, form),
            pb1: NewObject!(TPaintBox, form),
            listBox1: NewObject!(TListBox, form),
            trayicon: NewObject!(TTrayIcon, form),
            form,
        };
    }

    pub fn init(&self) -> &Self {
        println!("size TMessage: {}", size_of::<TMessage>());

        Application.SetOnMinimize(self, Self::onAppMinimize);

        // TForm
        self.form
            .SetCaption("你好，Rust！ - Hello Rust!")
            .SetHeight(556)
            .SetPosition(TPosition::poScreenCenter)
            .SetAllowDropFiles(true)
            .SetOnDropFiles(self, Self::onDropFile)
            .SetOnMouseMove(self, Self::onFormMouseMove)
            .SetColor(clSkyblue)
            .SetKeyPreview(true)
            .SetOnKeyDown(self, Self::onFormKeyDown)
            .SetOnDestroy(self, Self::onFormDestroy)
            .SetOnMouseDown(self, Self::onFormMouseDown)
            // .SetOnWndProc(self, Self::onFormWndProc)
            .SetOnMouseUp(self, Self::onFormMouseUp);

        // 测试自动drop
        // let ico = TIcon::new();
        // println!("{:?}", ico.ClassName());

        // TOpenDialog
        self.dlgOpen
            .SetFilter("Rust File|*.rs|Other|*.txt;*.c;*.go|All|*.*")
            .SetTitle("Open File");

        // TButton
        self.btn
            .SetParent(self)
            .SetLeft(10)
            .SetTop(50)
            .SetCaption("button1")
            .SetOnClick(self, Self::onBtnClick);

        // TButton
        self.btn2
            .SetParent(self)
            .SetLeft(10)
            .SetTop(self.btn.Top() + self.btn.Height() + 10)
            .SetWidth(120)
            .SetCaption("InputCombo")
            .SetOnClick(self, Self::onBtn2Click);

        // TButton
        self.btn3
            .SetParent(self)
            .SetLeft(10)
            .SetTop(self.btn2.Top() + self.btn2.Height() + 10)
            .SetCaption("MsgBox")
            .SetOnClick(self, Self::onBtn3Click);

        // TButton
        self.btnOpenDialog
            .SetParent(self)
            .SetLeft(10)
            .SetTop(self.btn3.Top() + self.btn3.Height() + 10)
            .SetWidth(120)
            .SetCaption("Open Dialog")
            .SetOnClick(self, Self::onBtnOpenDialogClick);

        // TButton
        self.btnColorDialog
            .SetParent(self)
            .SetLeft(10)
            .SetTop(self.btnOpenDialog.Top() + self.btnOpenDialog.Height() + 10)
            .SetWidth(150)
            .SetCaption("Open Color Dialog")
            .SetOnClick(self, Self::onBtnColorDialogClick)
            .SetShowHint(true)
            .SetHint("this a TButton");

        // TEdit
        self.edit1
            .SetParent(self)
            .SetBounds(
                10,
                self.btnColorDialog.Top() + self.btnColorDialog.Height() + 10,
                300,
                28,
            )
            .SetOnChange(self, Self::onEdit1Change)
            .SetTextHint("example: xxxx");

        // TMemo
        self.memo1
            .SetParent(self)
            .SetAlign(TAlign::alRight)
            .SetWidth(350)
            // 左边相对edit1 + 15距离
            .AnchorToNeighbour(TAnchorKind::akLeft, 15, &self.edit1)
            .SetScrollBars(TScrollStyle::ssAutoVertical)
            .Font()
            .SetSize(11)
            .SetName("Courier New");

        // TButton
        self.btnOpenForm2
            .SetParent(self)
            .SetLeft(10)
            .SetCaption("Open Form2")
            .SetWidth(120)
            .SetTop(self.edit1.Top() + self.edit1.Height() + 10);

        // TButton
        self.btnOpenForm3
            .SetParent(self)
            .SetLeft(self.btnOpenForm2.Left() + self.btnOpenForm2.Width() + 10)
            .SetCaption("Open Form3")
            .SetWidth(120)
            .SetTop(self.edit1.Top() + self.edit1.Height() + 10);

        // TMainMenu
        let fileItem = TMenuItem::new(self);
        fileItem.SetCaption("&File");
        let mut fSubItem = TMenuItem::new(self);
        // new
        fSubItem.SetCaption("&New");
        fSubItem.SetShortCutText("Ctrl+N");
        fSubItem.SetName("nFileNew");
        fSubItem.SetOnClick(self, Self::onMenuItemClick);
        //fSubItem.SetShortCut(TextToShortCut("Ctrl+N"));
        fileItem.Add(&fSubItem);
        // open
        fSubItem = TMenuItem::new(self);
        fSubItem.SetCaption("&Open");
        fSubItem.SetShortCutText("Ctrl+O");
        fSubItem.SetName("nFileOpen");
        fSubItem.SetOnClick(self, Self::onMenuItemClick);
        fileItem.Add(&fSubItem);
        // -
        fSubItem = TMenuItem::new(self);
        fSubItem.SetCaption("-");
        fileItem.Add(&fSubItem);
        // exit
        fSubItem = TMenuItem::new(self);
        fSubItem.SetCaption("&Exit");
        fSubItem.SetShortCutText("Ctrl+Q");
        fSubItem.SetName("nFileExit");
        fSubItem.SetOnClick(self, Self::onMenuItemClick);
        fileItem.Add(&fSubItem);

        self.mainMenu.Items().Add(&fileItem);

        // TPopupMenu
        // 设置form右键显示菜单
        self.form.SetPopupMenu(&self.popupMenu);

        fSubItem = TMenuItem::new(self);
        fSubItem.SetCaption("&Exit");
        fSubItem.SetShortCutText("Ctrl+Q");
        fSubItem.SetName("pmExit");
        fSubItem.SetOnClick(self, Self::onMenuItemClick);
        self.popupMenu.Items().Add(&fSubItem);

        // TTreeView
        self.tv1
            .SetParent(self)
            .SetLeft(10)
            .SetTop(self.btnOpenForm2.Top() + self.btnOpenForm2.Height() + 10)
            .SetWidth(300)
            .SetHeight(100)
            .SetOnClick(self, Self::onTv1Click);

        let node = self.tv1.Items().AddChild(&TTreeNode::Nil(), "First");
        self.tv1.Items().AddChild(&node, "Sec");
        node.Expand(true);

        // TPaintBox
        self.pb1
            .SetParent(self)
            .SetLeft(self.btnColorDialog.Left() + self.btnColorDialog.Width() + 10)
            .SetTop(10)
            .SetWidth(150)
            .SetHeight(200)
            .SetOnPaint(self, Self::onPb1Paint);

        // TListBox
        self.listBox1
            .SetParent(self)
            .SetLeft(10)
            .SetTop(self.tv1.Top() + self.tv1.Height() + 10)
            .SetWidth(300)
            .SetHeight(100)
            .SetItemHeight(30)
            .SetOnClick(self, Self::onListBox1Click)
            .SetOnDrawItem(self, Self::onListBox1CustomDraw);

        // 因为liblcl没有支持ASLR，所以MSVC编译器编译的会出现这个，使用link器参数关闭：/DYNAMICBASE:NO
        //#[cfg(not(all(target_env = "msvc", target_os = "windows", target_arch = "x86_64")))]
        self.listBox1.SetStyle(TListBoxStyle::lbOwnerDrawFixed);

        for i in 0..20 {
            let text = format!("item={}", i);
            self.listBox1.Items().Add(&text);
        }

        // TTrayIcon
        self.trayicon
            .SetVisible(true)
            .SetHint("hello!")
            .SetOnClick(self, Self::onTrayIconClick);

        // TMemoryStream
        let mem = TMemoryStream::new(); // 非组件类已实现drop方法，所以Free不是必须的，当然，你也可以手动调用Free
        let testArr: [i8; 6] = [1, 3, 6, 4, 5, 6];
        println!(
            "write len: {}", // sizeof(T) * testArr.len()
            mem.Write(testArr.as_ptr(), testArr.len() as i32)
        );

        println!("write byte: {}", mem.WriteByte(29));

        println!("stream pos: {}", mem.Position());
        mem.SetPosition(0);
        println!("stream pos: {}", mem.Position());

        println!("stream size: {}", mem.Size());
        // read
        let testArrRead: [i8; 6] = [0; 6];
        println!(
            "read len: {}",
            mem.Read(
                // sizeof(T) * testArr.len()
                testArr.as_ptr(),
                testArrRead.len() as i32
            )
        );
        println!("testArrRead: {:?}", &testArrRead);
        println!("read byte: {}", mem.ReadByte());

        let testI32Arr: [i32; 9] = [12, 28, 34324, 43432, 23434, 43242, 4342, 434, 433];
        mem.WriteArray(&testI32Arr);
        println!("stream size2: {}", mem.Size());

        mem.SetPosition(mem.Position() - (testI32Arr.len() * 4) as i64);
        let testI32ReadArray: [i32; 9] = [0; 9];
        println!("read len: {}", mem.ReadArray(&testI32ReadArray));
        println!("array {:?}", &testI32ReadArray);

        // mem.Read();

        return &self;
    }

    fn onTrayIconClick(&self, _sender: usize) {
        println!("trayIcon Click!");
        // 显示后位置会发生变化，需要找找原因。。。。
        self.form.Show();
        Application.Restore();
    }

    fn onFormWndProc(&self, msg: *mut TMessage) {
        unsafe {
            self.form.InheritedWndProc(msg);
            match (*msg).msg {
                WM_LBUTTONDOWN => println!("onFormWndProc: mouse down"), // WM_LBUTTONDOWN
                WM_LBUTTONUP => println!("onFormWndProc: mouse up"),     // WM_LBUTTONUP
                _ => return,
            }
        }
    }

    fn onListBox1CustomDraw(
        &self,
        _control: usize,
        index: i32,
        aRect: *mut TRect,
        state: TOwnerDrawState,
    ) {
        let canvas = self.listBox1.Canvas();
        let s = self.listBox1.Items().Strings(index).into_owned();
        let fw = canvas.TextWidth(&s);
        let fh = canvas.TextHeight(&s);
        let rect = unsafe { &*aRect };
        canvas.Font().SetColor(clBlack);
        canvas.Brush().SetColor(clBtnFace);
        canvas.FillRect(&rect);
        canvas.Brush().SetColor(0x00FFF7F7);
        canvas.Pen().SetColor(clSkyblue);
        canvas.Rectangle(rect.left + 1, rect.top + 1, rect.right - 1, rect.bottom - 1);
        canvas.Rectangle(rect.left, rect.top, rect.right, rect.bottom);
        if InSet!(state, TOwnerDrawStateType::odSelected) {
            canvas.Brush().SetColor(0x00FFB2B5);
            canvas.Rectangle(rect.left + 1, rect.top + 1, rect.right - 1, rect.bottom - 1);
            canvas.Font().SetColor(clBlue);
            if InSet!(state, TOwnerDrawStateType::odFocused) {
                canvas.DrawFocusRect(&rect);
            }
        }
        canvas.TextOut(
            rect.left + (rect.right - fw) / 2,
            rect.top + (30 - fh) / 2,
            &s,
        );
    }

    fn onListBox1Click(&self, _sender: usize) {
        let idx = self.listBox1.ItemIndex();
        if idx != -1 {
            println!("current item: {:}", self.listBox1.Items().Strings(idx));
        }
    }

    fn onPb1Paint(&self, _sender: usize) {
        let canvas = self.pb1.Canvas();
        let brush = canvas.Brush();

        let r = TRect {
            left: 10,
            top: 12,
            right: 80,
            bottom: 80,
        };
        brush.SetColor(clGreen);
        canvas.FillRect(&r);

        brush.SetStyle(TBrushStyle::bsClear);
        let cliRect = self.pb1.ClientRect();
        canvas.Pen().SetColor(clRed);
        canvas.Rectangle(cliRect.left, cliRect.top, cliRect.right, cliRect.bottom);

        let ico = Application.Icon(); //self.form.Icon();
        if !ico.IsNil() {
            brush.SetStyle(TBrushStyle::bsClear);
            // let jpg = TJPEGImage::new();
            // jpg.LoadFromFile("btn4[1].jpg");
            canvas.Draw1(10, 10, &ico);
        }

        brush.SetStyle(TBrushStyle::bsClear);
        let text = "Test darw text! 你好！";
        let font = canvas.Font();
        font.SetStyle(0).SetSize(9).SetColor(clBlue);
        let mut r2 = cliRect.clone();
        canvas.TextRect2(
            &mut r2,
            &text,
            Include!(
                0,
                TTextFormats::tfCenter,
                TTextFormats::tfVerticalCenter,
                TTextFormats::tfSingleLine
            ),
        );
    }

    fn onTv1Click(&self, _sender: usize) {
        let node = self.tv1.Selected();
        if !node.IsNil() {
            println!("node.text = {:}", node.Text());
        } else {
            println!("node is nil.");
        }
    }

    fn onFormMouseDown(
        &self,
        _sender: usize,
        button: TMouseButton,
        _shift: TShiftState,
        x: i32,
        y: i32,
    ) {
        println!("Button:{}, X:{} Y:{}", button == TMouseButton::mbLeft, x, y);
        println!("OnMouseDown");
    }

    fn onFormMouseUp(
        &self,
        _sender: usize,
        button: TMouseButton,
        _shift: TShiftState,
        x: i32,
        y: i32,
    ) {
        println!("Button:{}, X:{} Y:{}", button == TMouseButton::mbLeft, x, y);
        println!("OnMouseUp");
    }

    fn onMenuItemClick(&self, sender: usize) {
        let item = TMenuItem::from(sender);
        let name = item.Name();
        if name == "nFileNew" {
            self.memo1.Clear();
        } else if name == "nFileOpen" {
            Self::onBtnOpenDialogClick(self, sender);
        } else if name == "nFileExit" || name == "pmExit" {
            Application.Terminate();
        }
    }

    fn onEdit1Change(&self, _sender: usize) {
        println!("edit1.Change={:}", self.edit1.Text());
    }

    // 按钮1单击事件
    fn onBtnClick(&self, sender: usize) {
        ShowMessage("Hello, Rust! 你好，世界！");
        ShowMessageFmt!("Hello 1={}, 2={}", 1, "2");
        let btn = TButton::from(sender);
        println!("caption: {:?}", btn.Caption());

        let result = SelectDirectory(Include!(0, TSelectDirOpt::sdPrompt), 0);
        if result.0 {
            println!("SelectDirectory={:}", result.1);
        }
        let result2 = SelectDirectory2("caption", "C:\\", true);
        if result2.0 {
            println!("SelectDirectory2={:}", result2.1);
        }

        let result3 = InputQuery("Caption", "Prompt", "322");
        if result3.0 {
            println!("InputQuery={:}", result3.1);
        }
    }

    fn onBtn2Click(&self, _sender: usize) {
        //TStrings是抽像类，当参数类型为TStrings要用TStringList传入，非TComponent实现了drop方法，所以Free不是必须的
        let list = TStringList::new();
        list.Add("Item1");
        list.Add("Item2");
        list.Add("Item3");
        let idx = InputCombo(
            "caption",
            "Prompt                          ", // aPrompt参数决定显示的窗口宽度
            &list,
        );
        println!("select index: {}", idx);
    }

    fn onBtn3Click(&self, _sender: usize) {
        // 集合测试
        // 加法
        let val1 = Include!(0, 1, 2, 3); // result=14
        println!("val1={}", val1);
        // 减法
        let val2 = Exclude!(val1, 2); // result=10
        println!("val2={}", val2);

        println!("2 in set={}", InSet!(val2, 2));
        println!("3 in set={}", InSet!(val2, 3));

        ShowMessage("消息");
        if MessageDlg(
            "消息",
            TMsgDlgType::mtConfirmation,
            Include!(0, TMsgDlgBtn::mbYes, TMsgDlgBtn::mbNo),
            0,
        ) == mrYes
        {
            ShowMessage("你点击了“是")
        }
        // windows consts
        // if Application.MessageBox("消息", "标题", MB_OKCANCEL | MB_ICONINFORMATION) == idOK  {
        //     ShowMessage("你点击了“是")
        // }
    }

    fn onBtnOpenDialogClick(&self, _sender: usize) {
        if self.dlgOpen.Execute() {
            let fileName = self.dlgOpen.FileName();
            println!("fileName={:?}", &fileName);
            self.edit1.SetText(&fileName);
            self.memo1.Lines().LoadFromFile(&fileName);
        }
    }

    fn onBtnColorDialogClick(&self, _sender: usize) {
        if self.dlgColor.Execute() {
            let color = self.dlgColor.Color();
            println!("color={}", color);
            self.form.SetColor(color);
        }
    }

    fn onGetCellHint(
        &self,
        _sender: usize,
        _ACol: i32,
        _ARow: i32,
        hintText: *mut *const std::os::raw::c_char,
    ) {
        unsafe {
            *hintText = toCString("fff"); //??????
        }
    }

    fn onGetCheckboxState(
        &self,
        _sender: usize,
        _ACol: i32,
        _ARow: i32,
        _value: *mut TCheckBoxState,
    ) {
        unsafe {
            let state = TCheckBoxState::cbChecked;

            //*value = state;
        }
    }

    fn onDropFile(&self, _sender: usize, fileNames: usize, len: isize) {
        //println!("{}, {}, {}", _sender, fileNames, len);
        for i in 0..len {
            let fileName = GetFPStringArrayMember(fileNames, i);
            //println!("{}: {:?}", i, &fileName);
            self.memo1.Lines().Add(&fileName);
        }
    }

    fn onFormMouseMove(&self, _sender: usize, shift: TShiftState, _x: i32, _y: i32) {
        if InSet!(shift, TShiftStateEnum::ssCtrl) {
            println!("ctrl");
        }
        // let form = TForm::As(_sender);
        //let pos = Mouse.CursorPos();
        //println!("pos={:?}", &pos);
    }

    fn onFormKeyDown(&self, _sender: usize, key: *mut Char, _shift: TShiftState) {
        unsafe {
            println!("key down={}", *key);
            if *key == vkReturn {
                ShowMessage("down Enter!");
            }
        }
    }

    fn onFormDestroy(&self, _sender: usize) {
        println!("TMainForm destroy.");
    }

    fn onAppMinimize(&self, _sender: usize) {
        println!("AppMinimize");
        self.form.Hide();
    }
}
