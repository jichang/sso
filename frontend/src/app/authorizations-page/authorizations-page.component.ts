import { Component, OnInit } from '@angular/core';
import { Authorization, AuthorizationModelService } from '../authorization-model.service';
import { session } from '../model';

@Component({
  selector: 'authorizations-page',
  templateUrl: './authorizations-page.component.html',
  styleUrls: ['./authorizations-page.component.css']
})
export class AuthorizationsPageComponent implements OnInit {
  authorizations: Authorization[] = [];

  constructor(private authorizationModel: AuthorizationModelService) { }

  ngOnInit() {
    this.authorizationModel.authorizations.subscribe(authorizations => {
        this.authorizations = authorizations;
      });

    this.authorizationModel.select();
  }

  openCreateModal() {}
}
