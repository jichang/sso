import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Observable, BehaviorSubject } from 'rxjs'
import {session} from './model'

export interface Scope {
  id?: number;
  name: string;
  description: number;
  status: number;
}

export interface ScopeStore {
  scopes: Scope[]
}

@Injectable()
export class ScopeModelService {
  private store: ScopeStore;
  private subject: BehaviorSubject<Scope[]>;

  constructor(private http: HttpClient) {
    this.store = {
      scopes: []
    };
    this.subject = new BehaviorSubject<Scope[]>([]);
  }

  get scopes() {
    return this.subject.asObservable();
  }

  select(applicationId) {
    let headers = new HttpHeaders({
      'Content-Type': 'application/json',
      'Authorization': 'Bearer ' + window.localStorage.getItem('jwt')
    });
    let options = {
      headers: headers
    };

    let apiUri = "/api/v1/users/" + session.currUser().id + "/applications/" + applicationId + "/scopes";
    this.http.get(apiUri, options)
      .subscribe((scopes: Scope[]) => {
        this.store.scopes = scopes;
        this.subject.next(scopes);
      });
  }

  create(applicationId, scope: Scope) {
    let headers = new HttpHeaders({
      'Content-Type': 'application/json',
      'Authorization': 'Bearer ' + window.localStorage.getItem('jwt')
    });
    let options = {
      headers: headers
    };

    let apiUri = '/api/v1/users/' + session.currUser().id + '/applications/' + applicationId + '/scopes';
    return this.http.post(apiUri, scope, options)
      .map((scope: Scope) => {
        this.store.scopes.push(scope);
        this.subject.next(Object.assign({}, this.store).scopes);

        return scope;
      });
  }
}
