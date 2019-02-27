import { Contact } from "./contact-model.service";

export interface PaginatorParams {
  offset: number;
  limit: number;
}

export enum GenderType {
  Male = 1,
  Female,
  Others
}

export interface Gender {
  id: number;
  name: string;
  status: number;
}

export interface Profile {
  name: string;
  gender: Gender;
  birthday: number;
  introduction: string;
  status: number;
}

export interface Role {
  name: string;
}

export interface User {
  id?: number;
  role: Role;
  username: string;
  password: string;
  contacts: Contact[];
  status: number;
}

export interface ResourceCollection<T> {
  total: number;
  items: T[];
}

export let session = {
  currUser: function() {
    return JSON.parse(window.localStorage.getItem("currUser")) as User;
  },
  destory: function() {
    window.localStorage.removeItem("currUser");
    window.localStorage.removeItem("jwt");
  }
};
