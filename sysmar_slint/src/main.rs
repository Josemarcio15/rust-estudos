
use slint::*;
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}, Mutex, Weak, Rc};
use std::rc::Rc as StdRc;
use std::cell::RefCell;
use std::sync::mpsc;

slint::include_modules!(); // gera o módulo `ui`

// ----------- Modelos -------------------------------------------------------
#[derive(Clone, Debug, PartialEq, slint::ModelData)]
pub struct Student {
    pub id: usize,
    pub name: SharedString,
    pub email: SharedString,
    pub phone: SharedString,
    pub plan: SharedString,
    pub fee: f64,
    pub paid: bool,
}

// ----------- Componente principal -----------------------------------------
#[derive(Default)]
struct AppWindow {
    // propriedades que serão ligadas ao .slint
    students: VecModel<Student>,
    total_arrecadado: f64,
    display_diaria: SharedString,
    input_diaria: SharedString,
    display_mensal: SharedString,
    input_mensal: SharedString,
    display_semestral: SharedString,
    input_semestral: SharedString,
    display_anual: SharedString,
    input_anual: SharedString,
    show_edit_dialog: bool,
    edit_mode: bool,
    edit_index: i32,
    edit_name: SharedString,
    edit_email: SharedString,
    edit_phone: SharedString,
    edit_plan: SharedString,
    edit_fee_str: SharedString,
    edit_paid: bool,
}

// ----------- Funções auxiliares -------------------------------------------
fn parse_fee(s: &str) -> Option<f64> {
    s.parse::<f64>().ok()
}

// ----------- Implementação do `Widget` ------------------------------------
slint::component::impl_appwindow! {
    // propriedades declaradas no .slint serão expondo aqui
    #[upcast]
    pub struct AppWindow;

    // callbacks conectados ao UI (gerados a partir do .slint)
    // (os nomes são exatamente os mesmos definidos no .slint)
    fn open_add(&self) {
        self.show_edit_dialog = true;
        self.edit_mode = false;
        self.edit_name = "".into();
        self.edit_email = "".into();
        self.edit_phone = "".into();
        self.edit_plan = "mensal".into();
        self.edit_fee_str = "0.00".into();
        self.edit_paid = false;
    }

    fn open_edit(&self, index: i32) {
        if let Some(st) = self.students.row_data(index as usize) {
            self.show_edit_dialog = true;
            self.edit_mode = true;
            self.edit_index = index;
            self.edit_name = st.name.clone();
            self.edit_email = st.email.clone();
            self.edit_phone = st.phone.clone();
            self.edit_plan = st.plan.clone();
            self.edit_fee_str = format!("{:.2}", st.fee).into();
            self.edit_paid = st.paid;
        }
    }

    fn add_student(&mut self,
        name: SharedString,
        email: SharedString,
        phone: SharedString,
        plan: SharedString,
        fee_str: SharedString,
        paid: bool) {
        if let Ok(fee) = parse_fee(&fee_str) {
            let id = NEXT_ID.fetch_add(1, Ordering::SeqCst);
            let new_st = Student {
                id,
                name,
                email,
                phone,
                plan,
                fee,
                paid,
            };
            self.students.push(new_st);
        }
    }

    fn edit_student(&mut self,
        index: i32,
        name: SharedString,
        email: SharedString,
        phone: SharedString,
        plan: SharedString,
        fee_str: SharedString,
        paid: bool) {
        if let Ok(fee) = parse_fee(&fee_str) {
            let mut updated = self.students.row_data(index as usize).clone();
            updated.name = name;
            updated.email = email;
            updated.phone = phone;
            updated.plan = plan;
            updated.fee = fee;
            updated.paid = paid;
            self.students.set_row_data(index as usize, updated);
        }
    }

    fn delete_student(&mut self, id: usize) {
        for i in 0..self.students.row_count() {
            if let Some(st) = self.students.row_data(i) {
                if st.id == id {
                    self.students.remove(i);
                    break;
                }
            }
        }
    }

    fn pay_student(&mut self, id: usize) {
        let mut total = self.total_arrecadado;
        for i in 0..self.students.row_count() {
            if let Some(mut st) = self.students.row_data(i) {
                if st.id == id {
                    st.paid = true;
                    self.students.set_row_data(i, st);
                    total += st.fee;
                    self.total_arrecadado = total;
                    self.update_total_ui();
                    break;
                }
            }
        }
    }

    fn update_diaria(&mut self, txt: SharedString) {
        if let Ok(v) = txt.as_str().parse::<f64>() {
            SELF_RATES.lock().unwrap()[0] = v;
            self.display_diaria = format!("R$ {:.2}", v).into();
        }
    }

    fn update_mensal(&mut self, txt: SharedString) {
        if let Ok(v) = txt.as_str().parse::<f64>() {
            SELF_RATES.lock().unwrap()[1] = v;
            self.display_mensal = format!("R$ {:.2}", v).into();
        }
    }

    fn update_semestral(&mut self, txt: SharedString) {
        if let Ok(v) = txt.as_str().parse::<f64>() {
            SELF_RATES.lock().unwrap()[2] = v;
            self.display_semestral = format!("R$ {:.2}", v).into();
        }
    }

    fn update_anual(&mut self, txt: SharedString) {
        if let Ok(v) = txt.as_str().parse::<f64>() {
            SELF_RATES.lock().unwrap()[3] = v;
            self.display_anual = format!("R$ {:.2}", v).into();
        }
    }

    // ---------- Atualização da UI ----------
    fn update_total_ui(&mut self) {
        // a UI tem um campo chamado `total_arrecadado`
        self.total_arrecadado = *TOTAL_ARRECADADO.lock().unwrap();
        self.send_update(TotalArrecadadoUpdate(self.total_arrecadado));
    }
}

// ----------- Estruturas de estado global ---------------------------------
static NEXT_ID: AtomicUsize = AtomicUsize::new(1);
static SELF_RATES: StdRc<Mutex<Vec<f64>>> = StdRc::new(Mutex::new(vec![
    10.0, // diária
    100.0, // mensal
    500.0, // semestral
    1000.0, // anual
]));
static TOTAL_ARRECADADO: StdRc<Mutex<f64>> = StdRc::new(Mutex::new(0.0));

// ----------- Função `main` ------------------------------------------------
fn main() {
    // Cria a instância da janela
    let app = AppWindow::new().unwrap();

    // Modelo de alunos (vazio inicialmente)
    let model = VecModel::<Student>::default();
    let model_rc: ModelRc<VecModel<Student>> = ModelRc::new(model);
    app.set_students(model_rc.clone());

    // Taxas padrão (diária, mensal, semestral, anual)
    let rates = vec![10.0, 100.0, 500.0, 1000.0];
    app.set_display_diaria(format!("R$ {:.2}", rates[0]).into());
    app.set_display_mensal (format!("R$ {:.2}", rates[1]).into());
    app.set_display_semestral(format!("R$ {:.2}", rates[2]).into());
    app.set_display_anual   (format!("R$ {:.2}", rates[3]).into());

    // Referências fracas para evitar ciclos
    let model_weak = model_rc.as_weak();
    let app_weak = app.as_weak();

    // ---------- Nova inscrição ----------
    let next_id_clone = NEXT_ID.clone();
    let model_clone = model_rc.clone();
    app.on_open_add(move || {
        app_weak.unwrap().show_edit_dialog(true);
        app_weak.unwrap().edit_mode(false);
        app_weak.unwrap().edit_name("".into());
        app_weak.unwrap().edit_email("".into());
        app_weak.unwrap().edit_phone("".into());
        app_weak.unwrap().edit_plan("mensal".into());
        app_weak.unwrap().edit_fee_str("0.00".into());
        app_weak.unwrap().edit_paid(false);
    });

    // ---------- Abrir edição ----------
    let model_weak2 = model_rc.as_weak();
    let app_weak2 = app_weak.clone();
    app.on_open_edit(move |idx: i32| {
        let model = model_weak2.upgrade().unwrap();
        if let Some(st) = model.row_data(idx as usize) {
            let app = app_weak2.upgrade().unwrap();
            app.set_edit_name(st.name.clone());
            app.set_edit_email(st.email.clone());
            app.set_edit_phone(st.phone.clone());
            app.set_edit_plan(st.plan.clone());
            app.set_edit_fee_str(format!("{:.2}", st.fee).into());
            app.set_edit_paid(st.paid);
            app.set_edit_mode(true);
            app.set_edit_index(idx);
            app.set_show_edit_dialog(true);
        }
    });

    // ---------- Adicionar aluno ----------
    let next_id_c = NEXT_ID.clone();
    let model_c = model_rc.clone();
    app.on_add_student(move |name,email,phone,plan,fee_str,paid| {
        if let Ok(fee) = parse_fee(&fee_str) {
            let id = next_id_c.fetch_add(1, Ordering::SeqCst);
            let st = Student {
                id,
                name,
                email,
                phone,
                plan,
                fee,
                paid,
            };
            let model = model_c.clone();
            model.push(st);
        }
    });

    // ---------- Editar aluno ----------
    let model_c2 = model_rc.clone();
    app.on_edit_student(move |idx, name,email,phone,plan,fee_str,paid| {
        if let Ok(fee) = parse_fee(&fee_str) {
            let mut model = model_c2.clone();
            let mut st = model.row_data(idx as usize);
            st.name = name;
            st.email = email;
            st.phone = phone;
            st.plan = plan;
            st.fee = fee;
            st.paid = paid;
            model.set_row_data(idx as usize, st);
        }
    });

    // ---------- Excluir aluno ----------
    let model_c3 = model_rc.clone();
    app.on_delete_student(move |id: usize| {
        let mut model = model_c3.clone();
        for i in 0..model.row_count() {
            if let Some(st) = model.row_data(i) {
                if st.id == id {
                    model.remove(i);
                    break;
                }
            }
        }
    });

    // ---------- Pagar ----------
    let total_c = TOTAL_ARRECADADO.clone();
    let model_c4 = model_rc.clone();
    app.on_pay_student(move |id: usize| {
        let mut total = total_c.lock().unwrap();
        let mut model = model_c4.clone();
        for i in 0..model.row_count() {
            if let Some(mut st) = model.row_data(i) {
                if st.id == id {
                    st.paid = true;
                    model.set_row_data(i, st);
                    *total += st.fee;
                    app_weak.unwrap().set_total_arrecadado(*total);
                    break;
                }
            }
        }
    });

    // ---------- Atualização das taxas ----------
    let rates_clone = SELF_RATES.clone();
    app.on_update_diaria(move |txt: SharedString| {
        if let Ok(v) = txt.as_str().parse::<f64>() {
            let mut rates = rates_clone.lock().unwrap();
            rates[0] = v;
            drop(rates);
            app_weak.unwrap().set_display_diaria(format!("R$ {:.2}", v).into());
            app_weak.unwrap().set_input_diaria("".into());
        }
    });

    let rates_clone2 = SELF_RATES.clone();
    app.on_update_mensal(move |txt: SharedString| {
        if let Ok(v) = txt.as_str().parse::<f64>() {
            let mut rates = rates_clone2.lock().unwrap();
            rates[1] = v;
            drop(rates);
            app_weak.unwrap().set_display_mensal(format!("R$ {:.2}", v).into());
            app_weak.unwrap().set_input_mensal("".into());
        }
    });

    let rates_clone3 = SELF_RATES.clone();
    app.on_update_semestral(move |txt: SharedString| {
        if let Ok(v) = txt.as_str().parse::<f64>() {
            let mut rates = rates_clone3.lock().unwrap();
            rates[2] = v;
            drop(rates);
            app_weak.unwrap().set_display_semestral(format!("R$ {:.2}", v).into());
            app_weak.unwrap().set_input_semestral("".into());
        }
    });

    let rates_clone4 = SELF_RATES.clone();
    app.on_update_anual(move |txt: SharedString| {
        if let Ok(v) = txt.as_str().parse::<f64>() {
            let mut rates = rates_clone4.lock().unwrap();
            rates[3] = v;
            drop(rates);
            app_weak.unwrap().set_display_anual(format!("R$ {:.2}", v).into());
            app_weak.unwrap().set_input_anual("".into());
        }
    });

    // ---------- Inicializa total ----------
    app.set_total_arrecadado(0.0);

    // ---------- Executa ----------
    app.run().unwrap();
}