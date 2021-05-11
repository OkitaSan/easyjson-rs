use super::useful_kt_extensions::*;
struct Json{
    value:JsonValue
}
enum JsonValue{
    String(String),
    Number(f64),
    Object,
    Array(JsonArray),
    True,
    False,
    Null
}
struct JsonArray(Vec<Box<JsonValue>>);
impl std::ops::Deref for JsonArray{
    type Target = Vec<Box<JsonValue>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for JsonArray{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
struct JsonPair{
    key:String,
    value:Box<JsonValue>
}
struct JsonObject(Vec<JsonPair>);