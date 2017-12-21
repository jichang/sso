import { Component, OnInit } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Router, ActivatedRoute } from '@angular/router';
import { Contact, ContactType, ContactModelService } from '../contact-model.service';
import { session } from '../model';

enum ViewState {
  Create,
  Request,
  Verify
};

@Component({
  selector: 'verify-page',
  templateUrl: './verify-page.component.html',
  styleUrls: ['./verify-page.component.css']
})
export class VerifyPageComponent implements OnInit {
  State = ViewState;
  viewState: ViewState = ViewState.Create;

  constructor(private http: HttpClient, private route: ActivatedRoute) {
    let contact_id = this.route.snapshot.queryParams['contact_id'];
    let token = this.route.snapshot.queryParams["token"];

    if (contact_id && token) {
      this.viewState = ViewState.Verify;
    } else {
      this.viewState = ViewState.Request;
    }
  }

  ngOnInit() {
    if (this.viewState === ViewState.Verify) {
      let contact_id = this.route.snapshot.queryParams['contact_id'];
      let token = this.route.snapshot.queryParams["token"];
      let headers = new HttpHeaders({
        'Content-Type': 'application/json',
        'Authorization': 'Bearer ' + window.localStorage.getItem('jwt')
      });
      let options = {
        headers: headers
      };

      let user = session.currUser();
      let apiUri = '/api/v1/users/' + user.id + '/contacts/' + contact_id;
      this.http.post(apiUri, { token: token }, options)
        .subscribe((response: Response) => {
          console.log(response);
        });
    }
  }
}
