#![allow(unused_imports)]
#![allow(dead_code)]

pub mod Request {
  use std;
  use capnp::AnyPointer;
  use capnp::capability::{FromClientHook, FromTypelessPipeline};
  use capnp::{Text, Data};
  use capnp::layout;
  use capnp::layout::{FromStructBuilder, FromStructReader, ToStructReader};
  use capnp::{PrimitiveList, EnumList, StructList, TextList, DataList, ListList};
  use capnp::list::ToU16;

  pub static STRUCT_SIZE : layout::StructSize =
    layout::StructSize { data : 1, pointers : 2, preferred_list_encoding : layout::InlineComposite};


  pub struct Reader<'a> { reader : layout::StructReader<'a> }

  impl <'a> layout::FromStructReader<'a> for Reader<'a> {
    fn new(reader: layout::StructReader<'a>) -> Reader<'a> {
      Reader { reader : reader }
    }
  }

  impl <'a> layout::ToStructReader<'a> for Reader<'a> {
    fn struct_reader(&self) -> layout::StructReader<'a> { self.reader }
  }

  impl <'a> Reader<'a> {
    #[inline]
    pub fn get_key(&self) -> Text::Reader<'a> {
      self.reader.get_pointer_field(0).get_text(std::ptr::null(), 0)
    }
    pub fn has_key(&self) -> bool {
      !self.reader.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn get_operation(&self) -> ::messages_capnp::Request::Operation::Reader<'a> {
      FromStructReader::new(self.reader)
    }
  }

  pub struct Builder<'a> { builder : layout::StructBuilder<'a> }
  impl <'a> layout::HasStructSize for Builder<'a> {
    #[inline]
    fn struct_size(_unused_self : Option<Builder>) -> layout::StructSize { STRUCT_SIZE }
  }
  impl <'a> layout::FromStructBuilder<'a> for Builder<'a> {
    fn new(builder : layout::StructBuilder<'a>) -> Builder<'a> {
      Builder { builder : builder }
    }
  }
  impl <'a> Builder<'a> {
    pub fn as_reader(&self) -> Reader<'a> {
      FromStructReader::new(self.builder.as_reader())
    }
    #[inline]
    pub fn get_key(&self) -> Text::Builder<'a> {
      self.builder.get_pointer_field(0).get_text(std::ptr::null(), 0)
    }
    #[inline]
    pub fn set_key(&self, value : Text::Reader) {
      self.builder.get_pointer_field(0).set_text(value);
    }
    #[inline]
    pub fn init_key(&self, size : uint) -> Text::Builder<'a> {
      self.builder.get_pointer_field(0).init_text(size)
    }
    pub fn has_key(&self) -> bool {
      !self.builder.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn get_operation(&self) -> ::messages_capnp::Request::Operation::Builder<'a> {
      FromStructBuilder::new(self.builder)
    }
    #[inline]
    pub fn init_operation(&self, ) -> ::messages_capnp::Request::Operation::Builder<'a> {
      self.builder.set_data_field::<u16>(0, 0);
      self.builder.get_pointer_field(1).clear();
      FromStructBuilder::new(self.builder)
    }
  }

  pub struct Pipeline { _typeless : AnyPointer::Pipeline }
  impl FromTypelessPipeline for Pipeline {
    fn new(typeless : AnyPointer::Pipeline) -> Pipeline {
      Pipeline { _typeless : typeless }
    }
  }
  impl Pipeline {
    pub fn get_operation(&self) -> ::messages_capnp::Request::Operation::Pipeline {
      FromTypelessPipeline::new(self._typeless.noop())
    }
  }

  pub mod Operation {
    use std;
    use capnp::AnyPointer;
    use capnp::capability::{FromClientHook, FromTypelessPipeline};
    use capnp::{Text, Data};
    use capnp::layout;
    use capnp::layout::{FromStructBuilder, FromStructReader, ToStructReader};
    use capnp::{PrimitiveList, EnumList, StructList, TextList, DataList, ListList};
    use capnp::list::ToU16;

    pub struct Reader<'a> { reader : layout::StructReader<'a> }

    impl <'a> layout::FromStructReader<'a> for Reader<'a> {
      fn new(reader: layout::StructReader<'a>) -> Reader<'a> {
        Reader { reader : reader }
      }
    }

    impl <'a> layout::ToStructReader<'a> for Reader<'a> {
      fn struct_reader(&self) -> layout::StructReader<'a> { self.reader }
    }

    impl <'a> Reader<'a> {
      pub fn has_put(&self) -> bool {
        if self.reader.get_data_field::<u16>(0) != 2 { return false; }
        !self.reader.get_pointer_field(1).is_null()
      }
      #[inline]
      pub fn which(&self) -> std::option::Option<WhichReader<'a>> {
        match self.reader.get_data_field::<u16>(0) {
          0 => {
            return std::option::Some(Contains(
              ()
            ));
          }
          1 => {
            return std::option::Some(Get(
              ()
            ));
          }
          2 => {
            return std::option::Some(Put(
              self.reader.get_pointer_field(1).get_text(std::ptr::null(), 0)
            ));
          }
          _ => return std::option::None
        }
      }
    }

    pub struct Builder<'a> { builder : layout::StructBuilder<'a> }
    impl <'a> layout::FromStructBuilder<'a> for Builder<'a> {
      fn new(builder : layout::StructBuilder<'a>) -> Builder<'a> {
        Builder { builder : builder }
      }
    }
    impl <'a> Builder<'a> {
      pub fn as_reader(&self) -> Reader<'a> {
        FromStructReader::new(self.builder.as_reader())
      }
      #[inline]
      pub fn set_contains(&self, _value : ()) {
        self.builder.set_data_field::<u16>(0, 0);
      }
      #[inline]
      pub fn set_get(&self, _value : ()) {
        self.builder.set_data_field::<u16>(0, 1);
      }
      #[inline]
      pub fn set_put(&self, value : Text::Reader) {
        self.builder.set_data_field::<u16>(0, 2);
        self.builder.get_pointer_field(1).set_text(value);
      }
      #[inline]
      pub fn init_put(&self, size : uint) -> Text::Builder<'a> {
        self.builder.set_data_field::<u16>(0, 2);
        self.builder.get_pointer_field(1).init_text(size)
      }
      pub fn has_put(&self) -> bool {
        if self.builder.get_data_field::<u16>(0) != 2 { return false; }
        !self.builder.get_pointer_field(1).is_null()
      }
      #[inline]
      pub fn which(&self) -> std::option::Option<WhichBuilder<'a>> {
        match self.builder.get_data_field::<u16>(0) {
          0 => {
            return std::option::Some(Contains(
              ()
            ));
          }
          1 => {
            return std::option::Some(Get(
              ()
            ));
          }
          2 => {
            return std::option::Some(Put(
              self.builder.get_pointer_field(1).get_text(std::ptr::null(), 0)
            ));
          }
          _ => return std::option::None
        }
      }
    }

    pub struct Pipeline { _typeless : AnyPointer::Pipeline }
    impl FromTypelessPipeline for Pipeline {
      fn new(typeless : AnyPointer::Pipeline) -> Pipeline {
        Pipeline { _typeless : typeless }
      }
    }
    impl Pipeline {
    }
    pub enum Which<'a,A0> {
      Contains(()),
      Get(()),
      Put(A0),
    }
    pub type WhichReader<'a> = Which<'a,Text::Reader<'a>>;
    pub type WhichBuilder<'a> = Which<'a,Text::Builder<'a>>;
  }
}

pub mod Response {
  use std;
  use capnp::AnyPointer;
  use capnp::capability::{FromClientHook, FromTypelessPipeline};
  use capnp::{Text, Data};
  use capnp::layout;
  use capnp::layout::{FromStructBuilder, FromStructReader, ToStructReader};
  use capnp::{PrimitiveList, EnumList, StructList, TextList, DataList, ListList};
  use capnp::list::ToU16;

  pub static STRUCT_SIZE : layout::StructSize =
    layout::StructSize { data : 1, pointers : 2, preferred_list_encoding : layout::InlineComposite};


  pub struct Reader<'a> { reader : layout::StructReader<'a> }

  impl <'a> layout::FromStructReader<'a> for Reader<'a> {
    fn new(reader: layout::StructReader<'a>) -> Reader<'a> {
      Reader { reader : reader }
    }
  }

  impl <'a> layout::ToStructReader<'a> for Reader<'a> {
    fn struct_reader(&self) -> layout::StructReader<'a> { self.reader }
  }

  impl <'a> Reader<'a> {
    #[inline]
    pub fn get_key(&self) -> Text::Reader<'a> {
      self.reader.get_pointer_field(0).get_text(std::ptr::null(), 0)
    }
    pub fn has_key(&self) -> bool {
      !self.reader.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn which(&self) -> std::option::Option<WhichReader<'a>> {
      match self.reader.get_data_field::<u16>(1) {
        0 => {
          return std::option::Some(Contains(
            self.reader.get_bool_field(0)
          ));
        }
        1 => {
          return std::option::Some(Get(
            FromStructReader::new(self.reader)
          ));
        }
        2 => {
          return std::option::Some(Put(
            self.reader.get_bool_field(0)
          ));
        }
        _ => return std::option::None
      }
    }
  }

  pub struct Builder<'a> { builder : layout::StructBuilder<'a> }
  impl <'a> layout::HasStructSize for Builder<'a> {
    #[inline]
    fn struct_size(_unused_self : Option<Builder>) -> layout::StructSize { STRUCT_SIZE }
  }
  impl <'a> layout::FromStructBuilder<'a> for Builder<'a> {
    fn new(builder : layout::StructBuilder<'a>) -> Builder<'a> {
      Builder { builder : builder }
    }
  }
  impl <'a> Builder<'a> {
    pub fn as_reader(&self) -> Reader<'a> {
      FromStructReader::new(self.builder.as_reader())
    }
    #[inline]
    pub fn get_key(&self) -> Text::Builder<'a> {
      self.builder.get_pointer_field(0).get_text(std::ptr::null(), 0)
    }
    #[inline]
    pub fn set_key(&self, value : Text::Reader) {
      self.builder.get_pointer_field(0).set_text(value);
    }
    #[inline]
    pub fn init_key(&self, size : uint) -> Text::Builder<'a> {
      self.builder.get_pointer_field(0).init_text(size)
    }
    pub fn has_key(&self) -> bool {
      !self.builder.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn set_contains(&self, value : bool) {
      self.builder.set_data_field::<u16>(1, 0);
      self.builder.set_bool_field(0, value);
    }
    #[inline]
    pub fn init_get(&self, ) -> ::messages_capnp::Response::Get::Builder<'a> {
      self.builder.set_data_field::<u16>(1, 1);
      self.builder.set_data_field::<u16>(0, 0);
      self.builder.get_pointer_field(1).clear();
      FromStructBuilder::new(self.builder)
    }
    #[inline]
    pub fn set_put(&self, value : bool) {
      self.builder.set_data_field::<u16>(1, 2);
      self.builder.set_bool_field(0, value);
    }
    #[inline]
    pub fn which(&self) -> std::option::Option<WhichBuilder<'a>> {
      match self.builder.get_data_field::<u16>(1) {
        0 => {
          return std::option::Some(Contains(
            self.builder.get_bool_field(0)
          ));
        }
        1 => {
          return std::option::Some(Get(
            FromStructBuilder::new(self.builder)
          ));
        }
        2 => {
          return std::option::Some(Put(
            self.builder.get_bool_field(0)
          ));
        }
        _ => return std::option::None
      }
    }
  }

  pub struct Pipeline { _typeless : AnyPointer::Pipeline }
  impl FromTypelessPipeline for Pipeline {
    fn new(typeless : AnyPointer::Pipeline) -> Pipeline {
      Pipeline { _typeless : typeless }
    }
  }
  impl Pipeline {
  }
  pub enum Which<'a,A0> {
    Contains(bool),
    Get(A0),
    Put(bool),
  }
  pub type WhichReader<'a> = Which<'a,::messages_capnp::Response::Get::Reader<'a>>;
  pub type WhichBuilder<'a> = Which<'a,::messages_capnp::Response::Get::Builder<'a>>;

  pub mod Get {
    use std;
    use capnp::AnyPointer;
    use capnp::capability::{FromClientHook, FromTypelessPipeline};
    use capnp::{Text, Data};
    use capnp::layout;
    use capnp::layout::{FromStructBuilder, FromStructReader, ToStructReader};
    use capnp::{PrimitiveList, EnumList, StructList, TextList, DataList, ListList};
    use capnp::list::ToU16;

    pub struct Reader<'a> { reader : layout::StructReader<'a> }

    impl <'a> layout::FromStructReader<'a> for Reader<'a> {
      fn new(reader: layout::StructReader<'a>) -> Reader<'a> {
        Reader { reader : reader }
      }
    }

    impl <'a> layout::ToStructReader<'a> for Reader<'a> {
      fn struct_reader(&self) -> layout::StructReader<'a> { self.reader }
    }

    impl <'a> Reader<'a> {
      pub fn has_value(&self) -> bool {
        if self.reader.get_data_field::<u16>(0) != 0 { return false; }
        !self.reader.get_pointer_field(1).is_null()
      }
      #[inline]
      pub fn which(&self) -> std::option::Option<WhichReader<'a>> {
        match self.reader.get_data_field::<u16>(0) {
          0 => {
            return std::option::Some(Value(
              self.reader.get_pointer_field(1).get_text(std::ptr::null(), 0)
            ));
          }
          1 => {
            return std::option::Some(Empty(
              ()
            ));
          }
          _ => return std::option::None
        }
      }
    }

    pub struct Builder<'a> { builder : layout::StructBuilder<'a> }
    impl <'a> layout::FromStructBuilder<'a> for Builder<'a> {
      fn new(builder : layout::StructBuilder<'a>) -> Builder<'a> {
        Builder { builder : builder }
      }
    }
    impl <'a> Builder<'a> {
      pub fn as_reader(&self) -> Reader<'a> {
        FromStructReader::new(self.builder.as_reader())
      }
      #[inline]
      pub fn set_value(&self, value : Text::Reader) {
        self.builder.set_data_field::<u16>(0, 0);
        self.builder.get_pointer_field(1).set_text(value);
      }
      #[inline]
      pub fn init_value(&self, size : uint) -> Text::Builder<'a> {
        self.builder.set_data_field::<u16>(0, 0);
        self.builder.get_pointer_field(1).init_text(size)
      }
      pub fn has_value(&self) -> bool {
        if self.builder.get_data_field::<u16>(0) != 0 { return false; }
        !self.builder.get_pointer_field(1).is_null()
      }
      #[inline]
      pub fn set_empty(&self, _value : ()) {
        self.builder.set_data_field::<u16>(0, 1);
      }
      #[inline]
      pub fn which(&self) -> std::option::Option<WhichBuilder<'a>> {
        match self.builder.get_data_field::<u16>(0) {
          0 => {
            return std::option::Some(Value(
              self.builder.get_pointer_field(1).get_text(std::ptr::null(), 0)
            ));
          }
          1 => {
            return std::option::Some(Empty(
              ()
            ));
          }
          _ => return std::option::None
        }
      }
    }

    pub struct Pipeline { _typeless : AnyPointer::Pipeline }
    impl FromTypelessPipeline for Pipeline {
      fn new(typeless : AnyPointer::Pipeline) -> Pipeline {
        Pipeline { _typeless : typeless }
      }
    }
    impl Pipeline {
    }
    pub enum Which<'a,A0> {
      Value(A0),
      Empty(()),
    }
    pub type WhichReader<'a> = Which<'a,Text::Reader<'a>>;
    pub type WhichBuilder<'a> = Which<'a,Text::Builder<'a>>;
  }
}
