/*
    Slot types. Sketching ideas.


*/

struct PachaType <I,F> {
    name: String,
    id: usize,
    ty: PT_Descr<I,F>,
}

enum PT_Descr <I,F> {
    String(String),
    Integer(I),
    Float(F),
    Vector2([F; 2]),
    Vector3([F; 3]),
    Matrix3x3([[F;3];3]),
    Matrix4x4([[F;4];4]),
    //
    // Closure([PT_Toplevel], [PT_Toplevel]),
    // Structure([PT_Toplevel]),
}
