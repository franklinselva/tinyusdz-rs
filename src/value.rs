//! USD value types.

use half::f16;

/// USD value types enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum ValueType {
    Unknown = 0,
    Token,
    String,
    Bool,
    Half,
    Int,
    Uint,
    Int64,
    Uint64,
    Float,
    Double,
    Half2,
    Half3,
    Half4,
    Int2,
    Int3,
    Int4,
    Uint2,
    Uint3,
    Uint4,
    Float2,
    Float3,
    Float4,
    Double2,
    Double3,
    Double4,
    Quath,
    Quatf,
    Quatd,
    Color3h,
    Color3f,
    Color3d,
    Color4h,
    Color4f,
    Color4d,
    Point3h,
    Point3f,
    Point3d,
    Normal3h,
    Normal3f,
    Normal3d,
    Vector3h,
    Vector3f,
    Vector3d,
    TexCoord2h,
    TexCoord2f,
    TexCoord2d,
    TexCoord3h,
    TexCoord3f,
    TexCoord3d,
    Matrix2d,
    Matrix3d,
    Matrix4d,
    Frame4d,
}

/// A USD value that can hold various types.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum Value {
    /// No value.
    #[default]
    None,
    /// Boolean value.
    Bool(bool),
    /// 16-bit integer.
    Half(f16),
    /// 32-bit signed integer.
    Int(i32),
    /// 32-bit unsigned integer.
    Uint(u32),
    /// 64-bit signed integer.
    Int64(i64),
    /// 64-bit unsigned integer.
    Uint64(u64),
    /// 32-bit float.
    Float(f32),
    /// 64-bit float.
    Double(f64),

    // Vector types
    /// 2D half vector.
    Half2([f16; 2]),
    /// 3D half vector.
    Half3([f16; 3]),
    /// 4D half vector.
    Half4([f16; 4]),
    /// 2D integer vector.
    Int2([i32; 2]),
    /// 3D integer vector.
    Int3([i32; 3]),
    /// 4D integer vector.
    Int4([i32; 4]),
    /// 2D unsigned integer vector.
    Uint2([u32; 2]),
    /// 3D unsigned integer vector.
    Uint3([u32; 3]),
    /// 4D unsigned integer vector.
    Uint4([u32; 4]),
    /// 2D float vector.
    Float2([f32; 2]),
    /// 3D float vector.
    Float3([f32; 3]),
    /// 4D float vector.
    Float4([f32; 4]),
    /// 2D double vector.
    Double2([f64; 2]),
    /// 3D double vector.
    Double3([f64; 3]),
    /// 4D double vector.
    Double4([f64; 4]),

    // Quaternion types
    /// Half precision quaternion.
    Quath([f16; 4]),
    /// Float quaternion.
    Quatf([f32; 4]),
    /// Double quaternion.
    Quatd([f64; 4]),

    // Color types
    /// RGB color (half precision).
    Color3h([f16; 3]),
    /// RGB color (float).
    Color3f([f32; 3]),
    /// RGB color (double).
    Color3d([f64; 3]),
    /// RGBA color (half precision).
    Color4h([f16; 4]),
    /// RGBA color (float).
    Color4f([f32; 4]),
    /// RGBA color (double).
    Color4d([f64; 4]),

    // Spatial types
    /// 3D point (half precision).
    Point3h([f16; 3]),
    /// 3D point (float).
    Point3f([f32; 3]),
    /// 3D point (double).
    Point3d([f64; 3]),
    /// 3D normal (half precision).
    Normal3h([f16; 3]),
    /// 3D normal (float).
    Normal3f([f32; 3]),
    /// 3D normal (double).
    Normal3d([f64; 3]),
    /// 3D vector (half precision).
    Vector3h([f16; 3]),
    /// 3D vector (float).
    Vector3f([f32; 3]),
    /// 3D vector (double).
    Vector3d([f64; 3]),

    // Texture coordinate types
    /// 2D texture coordinate (half precision).
    TexCoord2h([f16; 2]),
    /// 2D texture coordinate (float).
    TexCoord2f([f32; 2]),
    /// 2D texture coordinate (double).
    TexCoord2d([f64; 2]),
    /// 3D texture coordinate (half precision).
    TexCoord3h([f16; 3]),
    /// 3D texture coordinate (float).
    TexCoord3f([f32; 3]),
    /// 3D texture coordinate (double).
    TexCoord3d([f64; 3]),

    // Matrix types
    /// 2x2 double matrix.
    Matrix2d([[f64; 2]; 2]),
    /// 3x3 double matrix.
    Matrix3d([[f64; 3]; 3]),
    /// 4x4 double matrix.
    Matrix4d([[f64; 4]; 4]),
    /// 4x4 frame matrix.
    Frame4d([[f64; 4]; 4]),

    // String types
    /// Token (interned string).
    Token(String),
    /// String value.
    String(String),

    // Array types
    /// Array of booleans.
    BoolArray(Vec<bool>),
    /// Array of half precision floats.
    HalfArray(Vec<f16>),
    /// Array of integers.
    IntArray(Vec<i32>),
    /// Array of unsigned integers.
    UintArray(Vec<u32>),
    /// Array of 64-bit integers.
    Int64Array(Vec<i64>),
    /// Array of 64-bit unsigned integers.
    Uint64Array(Vec<u64>),
    /// Array of floats.
    FloatArray(Vec<f32>),
    /// Array of doubles.
    DoubleArray(Vec<f64>),

    // Vector array types
    /// Array of 2D float vectors.
    Float2Array(Vec<[f32; 2]>),
    /// Array of 3D float vectors.
    Float3Array(Vec<[f32; 3]>),
    /// Array of 4D float vectors.
    Float4Array(Vec<[f32; 4]>),
    /// Array of 2D double vectors.
    Double2Array(Vec<[f64; 2]>),
    /// Array of 3D double vectors.
    Double3Array(Vec<[f64; 3]>),
    /// Array of 4D double vectors.
    Double4Array(Vec<[f64; 4]>),

    // Integer vector array types
    /// Array of 2D integer vectors.
    Int2Array(Vec<[i32; 2]>),
    /// Array of 3D integer vectors.
    Int3Array(Vec<[i32; 3]>),
    /// Array of 4D integer vectors.
    Int4Array(Vec<[i32; 4]>),

    // String arrays
    /// Array of tokens.
    TokenArray(Vec<String>),
    /// Array of strings.
    StringArray(Vec<String>),

    // Matrix arrays
    /// Array of 4x4 matrices.
    Matrix4dArray(Vec<[[f64; 4]; 4]>),
}

impl Value {
    /// Returns true if this value is an array type.
    pub fn is_array(&self) -> bool {
        matches!(
            self,
            Value::BoolArray(_)
                | Value::HalfArray(_)
                | Value::IntArray(_)
                | Value::UintArray(_)
                | Value::Int64Array(_)
                | Value::Uint64Array(_)
                | Value::FloatArray(_)
                | Value::DoubleArray(_)
                | Value::Float2Array(_)
                | Value::Float3Array(_)
                | Value::Float4Array(_)
                | Value::Double2Array(_)
                | Value::Double3Array(_)
                | Value::Double4Array(_)
                | Value::Int2Array(_)
                | Value::Int3Array(_)
                | Value::Int4Array(_)
                | Value::TokenArray(_)
                | Value::StringArray(_)
                | Value::Matrix4dArray(_)
        )
    }

    /// Returns the value as a bool, if it is one.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(v) => Some(*v),
            _ => None,
        }
    }

    /// Returns the value as an i32, if it is one.
    pub fn as_int(&self) -> Option<i32> {
        match self {
            Value::Int(v) => Some(*v),
            _ => None,
        }
    }

    /// Returns the value as an f32, if it is one.
    pub fn as_float(&self) -> Option<f32> {
        match self {
            Value::Float(v) => Some(*v),
            _ => None,
        }
    }

    /// Returns the value as an f64, if it is one.
    pub fn as_double(&self) -> Option<f64> {
        match self {
            Value::Double(v) => Some(*v),
            _ => None,
        }
    }

    /// Returns the value as a string slice, if it is a string or token.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Value::Token(s) | Value::String(s) => Some(s),
            _ => None,
        }
    }

    /// Returns the value as a Float3 array, if it is one.
    pub fn as_float3_array(&self) -> Option<&[[f32; 3]]> {
        match self {
            Value::Float3Array(v) => Some(v),
            _ => None,
        }
    }

    /// Returns the value as a Float2 array, if it is one.
    pub fn as_float2_array(&self) -> Option<&[[f32; 2]]> {
        match self {
            Value::Float2Array(v) => Some(v),
            _ => None,
        }
    }

    /// Returns the value as an Int array, if it is one.
    pub fn as_int_array(&self) -> Option<&[i32]> {
        match self {
            Value::IntArray(v) => Some(v),
            _ => None,
        }
    }

    /// Returns the value as a Float3 (single value), if it is one.
    pub fn as_float3(&self) -> Option<[f32; 3]> {
        match self {
            Value::Float3(v) | Value::Point3f(v) | Value::Normal3f(v) | Value::Vector3f(v) => {
                Some(*v)
            }
            Value::Color3f(v) => Some(*v),
            _ => None,
        }
    }

    /// Returns the value as a Float4 (single value), if it is one.
    pub fn as_float4(&self) -> Option<[f32; 4]> {
        match self {
            Value::Float4(v) | Value::Quatf(v) | Value::Color4f(v) => Some(*v),
            _ => None,
        }
    }

    /// Returns the value as a Matrix4d, if it is one.
    pub fn as_matrix4d(&self) -> Option<[[f64; 4]; 4]> {
        match self {
            Value::Matrix4d(m) | Value::Frame4d(m) => Some(*m),
            _ => None,
        }
    }
}

