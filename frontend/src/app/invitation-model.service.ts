import { Injectable } from "@angular/core";
import { BehaviorSubject } from "rxjs";
import { HttpHeaders, HttpClient, HttpParams } from "@angular/common/http";
import { PaginatorParams, User, ResourceCollection } from "./model";
import { map } from "rxjs/operators";

export interface Invitation {
  id: number;
  code: string;
  status: number;
}

export interface InvitationStore {
  invitations: Invitation[];
}

@Injectable()
export class InvitationModelService {
  private store: InvitationStore;
  private subject: BehaviorSubject<Invitation[]>;

  constructor(private http: HttpClient) {
    this.store = {
      invitations: []
    };
    this.subject = new BehaviorSubject<Invitation[]>([]);
  }

  get invitations() {
    return this.subject.asObservable();
  }

  create(userId: number) {
    let headers = new HttpHeaders({
      "Content-Type": "application/json"
    });
    let options = {
      headers: headers
    };

    let apiUri = `/api/v1/users/${userId}/invitations`;
    return this.http.post(apiUri, options).pipe(
      map((invitation: Invitation) => {
        this.store.invitations.push(invitation);
        this.subject.next([...this.store.invitations]);

        return invitation;
      })
    );
  }

  select(userId: number) {
    let headers = new HttpHeaders({
      "Content-Type": "application/json"
    });
    let options = {
      headers: headers
    };

    let apiUri = `/api/v1/users/${userId}/invitations`;
    return this.http.get(apiUri, options).subscribe(
      (invitations: Invitation[]) => {
        this.store.invitations = invitations;
        this.subject.next(invitations);
      },
      err => {
        this.subject.error(err);
      }
    );
  }

  remove(userId: number, invitation: Invitation) {
    let headers = new HttpHeaders({
      "Content-Type": "application/json"
    });
    let options = {
      headers: headers
    };

    let apiUri = `/api/v1/users/${userId}/invitations/${invitation.id}`;
    return this.http.delete(apiUri, options).pipe(
      map((invitation: Invitation) => {
        let index = this.store.invitations.findIndex(
          _invitation => invitation.id === _invitation.id
        );
        if (index !== -1) {
          this.store.invitations.splice(index, 1);
        }
        this.subject.next([...this.store.invitations]);

        return invitation;
      })
    );
  }
}
