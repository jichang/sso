import { Injectable } from "@angular/core";
import { BehaviorSubject } from "rxjs";
import { HttpClient, HttpHeaders } from "@angular/common/http";

export enum ResourceType {
  None = 0,
  Application = 1,
  Group = 2,
  GroupUser = 3,
  Permission = 4,
  RoleUser = 5,
  RolePermisson = 6,
  User = 7
}

export enum ActionType {
  NONE = 0,
  CREATE = 1,
  SELECT = 2,
  UPDATE = 3,
  DELETE = 4
}

export interface Permission {
  resource_type: number;
  action_type: number;
}

export interface PermissionStore {
  permissions: Permission[];
}

@Injectable()
export class PermissionModelService {
  private store: PermissionStore;
  private subject: BehaviorSubject<Permission[]>;

  constructor(private http: HttpClient) {
    this.store = {
      permissions: []
    };
    this.subject = new BehaviorSubject<Permission[]>([]);
  }

  get permissions() {
    return this.subject.asObservable();
  }

  select() {
    let headers = new HttpHeaders({
      "Content-Type": "application/json"
    });
    let options = {
      headers: headers
    };

    let apiUri = `/api/v1/permissions`;
    this.http.get(apiUri, options).subscribe(
      (permissions: Permission[]) => {
        this.store.permissions = permissions;
        this.subject.next(permissions);
      },
      err => {
        this.subject.error(err);
      }
    );
  }
}
