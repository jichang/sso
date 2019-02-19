import { Injectable } from "@angular/core";
import { Response, RequestOptions } from "@angular/http";
import { Observable, BehaviorSubject } from "rxjs";
import { map } from "rxjs/operators";
import { session } from "./model";
import { HttpClient, HttpHeaders, HttpParams } from "@angular/common/http";
import { Permission } from "./permission-model.service";

export interface Role {
  id: number;
  name: string;
  permissions: Permission[];
  status: number;
}

export interface RoleStore {
  roles: Role[];
}

@Injectable()
export class RoleModelService {
  private store: RoleStore;
  private subject: BehaviorSubject<Role[]>;

  constructor(private http: HttpClient) {
    this.store = {
      roles: []
    };
    this.subject = new BehaviorSubject<Role[]>([]);
  }

  get roles() {
    return this.subject.asObservable();
  }

  select() {
    let headers = new HttpHeaders({
      "Content-Type": "application/json"
    });
    let options = {
      headers: headers
    };

    let apiUri = `/api/v1/roles`;
    this.http.get(apiUri, options).subscribe(
      (roles: Role[]) => {
        this.store.roles = roles;
        this.subject.next(roles);
      },
      err => {
        this.subject.error(err);
      }
    );
  }

  grantPermission(role: Role, permission: Permission) {
    let headers = new HttpHeaders({
      "Content-Type": "application/json"
    });
    let options = {
      headers: headers
    };

    let apiUri = `/api/v1/roles/${role.id}/permissions`;
    return this.http.post(apiUri, permission, options);
  }

  revokePermission(role: Role, permission: Permission) {
    let params = new HttpParams();
    params = params.append(
      "resource_type",
      permission.resource_type.toString()
    );
    params = params.append("action_type", permission.action_type.toString());

    let apiUri = `/api/v1/roles/${role.id}/permissions`;
    return this.http.delete(apiUri, { params });
  }
}
