@0xe3743ae0973efd68;

struct Request {
  key @0 :Text;

  operation :union {
    contains @1 :Void;
    get @2 :Void;
    put @3 :Text;
  }
}

struct Response {
  key @0 :Text;

  union {
    contains @1 :Bool;
    get :union {
        value @2 :Text;
        empty @3 :Void;
    }
    put @4 :Bool;
  }
}
