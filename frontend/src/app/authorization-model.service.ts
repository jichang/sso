import { Injectable } from "@angular/core";
import { Observable, BehaviorSubject } from "rxjs";
import { Application } from "./application-model.service";
import { Scope } from "./scope-model.service";
import { session } from "./model";
import { HttpClient, HttpHeaders } from "@angular/common/http";
import { map } from "rxjs/operators";

export interface Authorization {
  id: number;
  client_app?: Application;
  server_app?: Application;
  scope?: Scope;
}

export interface AuthorizationStore {
  authorizations: Authorization[];
}

@Injectable()
export class AuthorizationModelService {
  private store: AuthorizationStore;
  private subject: BehaviorSubject<Authorization[]>;

  constructor(private http: HttpClient) {
    this.store = {
      authorizations: []
    };
    this.subject = new BehaviorSubject<Authorization[]>([]);
  }

  get authorizations() {
    return this.subject.asObservable();
  }

  select(userId: number) {
    let headers = new HttpHeaders({
      "Content-Type": "application/json",
      Authorization: "Bearer " + window.localStorage.getItem("jwt")
    });
    let options = {
      headers: headers
    };

    let apiUri = `/api/v1/users/${userId}/authorizations`;
    this.http
      .get(apiUri, options)
      .subscribe((authorizations: Authorization[]) => {
        this.store.authorizations = authorizations;
        this.subject.next(authorizations);
      });
  }

  create(authorization: Authorization) {
    let headers = new HttpHeaders({
      "Content-Type": "application/json",
      Authorization: "Bearer " + window.localStorage.getItem("jwt")
    });
    let options = {
      headers: headers
    };

    let apiUri = "/api/v1/users/" + session.currUser().id + "/authorizations";
    return this.http.post(apiUri, authorization, options).pipe(
      map((authorization: Authorization) => {
        this.store.authorizations.push(authorization);
        this.subject.next(Object.assign({}, this.store).authorizations);

        return authorization;
      })
    );
  }

  remove(authorization: Authorization) {
    let headers = new HttpHeaders({
      Authorization: "Bearer " + window.localStorage.getItem("jwt")
    });
    let options = {
      headers: headers
    };

    let apiUri = `/api/v1/users/${session.currUser().id}/authorizations/${
      authorization.id
    }`;

    return this.http.delete(apiUri, options).pipe(
      map((authorization: Authorization) => {
        let index = this.store.authorizations.findIndex(
          _authorization => _authorization.id === authorization.id
        );
        this.store.authorizations.splice(index, 1);

        this.subject.next(Object.assign({}, this.store).authorizations);

        return authorization;
      })
    );
  }
}
