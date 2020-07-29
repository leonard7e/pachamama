/*
    Slot types. Sketching ideas.
*/

/*
    This is how data is represented in pachamama. PType is a shortcut for Pachamama Type.
*/

struct PType <I,F> {
    name: String,
    id: usize,
    ty: PT_Descr<I,F>,
}

/*
    Type descriptor.
*/

enum PT_Descr <I,F> {
    String(String),
    Integer(I),
    Float(F),
    Vector2([F; 2]),
    Vector3([F; 3]),
    Matrix3x3([[F;3];3]),
    Matrix4x4([[F;4];4]),
    //
    // Closure([PT_Descr], [PT_Descr]),
    // Structure([PT_Descr]),
}
