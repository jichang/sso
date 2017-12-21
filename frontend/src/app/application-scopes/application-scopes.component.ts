import { Component, Input, OnInit } from '@angular/core';
import { Router, ActivatedRoute } from '@angular/router';
import { Scope, ScopeModelService } from '../scope-model.service';

@Component({
  selector: 'application-scopes',
  templateUrl: './application-scopes.component.html',
  styleUrls: ['./application-scopes.component.css']
})
export class ApplicationScopesComponent implements OnInit {
  scopes: Scope[] = [];

  constructor(private route: ActivatedRoute, private scopeModel: ScopeModelService) { }

  ngOnInit() {
    this.scopeModel.scopes.subscribe(scopes => {
        this.scopes = scopes;
      });

    this.scopeModel.select(this.route.parent.snapshot.params['id']);
  }

}
