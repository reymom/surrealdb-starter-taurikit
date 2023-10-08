import { ipc_invoke } from "./ipc";
import { Page, Person, PersonForCreate, PersonForUpdate } from "@/bindings";

class Controller<M, C, U> {
  suffix: string;

  constructor(suffix: string) {
    this.suffix = suffix;
  }

  async get(id: string): Promise<M> {
    return ipc_invoke(`get_${this.suffix}`, { id }).then((res) => res.data);
  }

  async create(data: C): Promise<String> {
    return ipc_invoke(`create_${this.suffix}`, { data }).then((res) => {
      return res.data;
    });
  }

  async update(id: string, data: U): Promise<String> {
    return ipc_invoke(`update_${this.suffix}`, { id, data }).then((res) => {
      return res.data;
    });
  }

  async delete(id: string): Promise<String> {
    return ipc_invoke(`delete_${this.suffix}`, { id }).then((res) => res.data);
  }
}

class PersonController extends Controller<
  Person,
  PersonForCreate,
  PersonForUpdate
> {
  constructor() {
    super("person");
  }

  async list(page: Page): Promise<Person[]> {
    return ipc_invoke(`list_${this.suffix}s`, { page }).then((res) => res.data);
  }
}
export const personController = new PersonController();
